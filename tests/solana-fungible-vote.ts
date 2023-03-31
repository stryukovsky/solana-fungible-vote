import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaFungibleVote } from "../target/types/solana_fungible_vote";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { expect } from "chai";

describe("solana-fungible-vote", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.SolanaFungibleVote as Program<SolanaFungibleVote>;
    const adminAccount = anchor.web3.Keypair.generate();
    const mint = anchor.web3.Keypair.generate();
    const adminAddress = anchor.getProvider().publicKey;

    const user1 = anchor.web3.Keypair.generate();
    const user2 = anchor.web3.Keypair.generate();

    const provider = anchor.getProvider();
    const connection = provider.connection;

    const votingAccount = anchor.web3.Keypair.generate();

    it("should be initialized", async () => {
        await program.methods.initialize().accounts({
            adminAccount: adminAccount.publicKey,
            mint: mint.publicKey,
        }).signers([adminAccount, mint]).rpc();
    });

    it("should register an account for admin", async () => {
        const tokenAccount = await getAssociatedTokenAddress(mint.publicKey, adminAddress);
        await program.methods.initializeAccount().accounts({
            tokenAccount,
            mint: mint.publicKey,
        }).rpc();
    });

    it("should give native assets to user1", async () => {
        const tx = new anchor.web3.Transaction().add(anchor.web3.SystemProgram.transfer({
            fromPubkey: adminAddress,
            toPubkey: user1.publicKey,
            lamports: anchor.web3.LAMPORTS_PER_SOL
        }));
        await provider.sendAndConfirm(tx);
    });

    it("should give native assets to user2", async () => {
        const tx = new anchor.web3.Transaction().add(anchor.web3.SystemProgram.transfer({
            fromPubkey: adminAddress,
            toPubkey: user2.publicKey,
            lamports: anchor.web3.LAMPORTS_PER_SOL,
        }));
        await provider.sendAndConfirm(tx);
    });


    it("should register an account for user 1", async () => {
        const tokenAccount = await getAssociatedTokenAddress(mint.publicKey, user1.publicKey);
        await program.methods.initializeAccount().accounts({
            tokenAccount,
            mint: mint.publicKey,
            authority: user1.publicKey,
        }).signers([user1]).rpc();
    });


    it("should register an account for user 2", async () => {
        const tokenAccount = await getAssociatedTokenAddress(mint.publicKey, user2.publicKey);
        await program.methods.initializeAccount().accounts({
            tokenAccount,
            mint: mint.publicKey,
            authority: user2.publicKey,
        }).signers([user2]).rpc();
    });

    const quorum = new anchor.BN(2000);
    const votingUntil = new anchor.BN(5 + Math.round((new Date().getTime()) / 1000));
    it("should start a new voting", async () => {
        const votingTokenAccount = await getAssociatedTokenAddress(mint.publicKey, votingAccount.publicKey);
        await program.methods.initializeVoting(quorum, votingUntil).accounts({
            votingAccount: votingAccount.publicKey,
            adminAccount: adminAccount.publicKey,
            mint: mint.publicKey,
            votingTokenAccount,
        }).signers([votingAccount]).rpc();
    });

    it("should accept vote in admin account", async () => {
        await program.methods.registerVoting().accounts({
            votingAccount: votingAccount.publicKey,
            adminAccount: adminAccount.publicKey,
        }).rpc();
    });

    it("should revert attempt add vote when current vote exists", async () => {
        try {
            await program.methods.registerVoting().accounts({
                votingAccount: votingAccount.publicKey,
                adminAccount: adminAccount.publicKey,
            }).rpc();
        }
        catch (e) {
            expect(e.error.errorMessage).eq("VotingAlreadyStarted");
        }
    });

    const votesAmount = quorum.div(new anchor.BN(2));
    it("should issue some vote tokens for voter user1", async () => {
        const recipient = await getAssociatedTokenAddress(mint.publicKey, user1.publicKey);
        await program.methods.issueVotes(votesAmount).accounts({
            mint: mint.publicKey,
            recipient: recipient,
        }).rpc();
    });

    it("should issue some vote tokens for voter user2", async () => {
        const recipient = await getAssociatedTokenAddress(mint.publicKey, user2.publicKey);
        await program.methods.issueVotes(votesAmount).accounts({
            mint: mint.publicKey,
            recipient: recipient,
        }).rpc();
    });

    it("should perform vote from voter user1", async () => {
        const voterTokenAccount = await getAssociatedTokenAddress(mint.publicKey, user1.publicKey);
        const votingTokenAccount = await getAssociatedTokenAddress(mint.publicKey, votingAccount.publicKey);
        await program.methods.vote(votesAmount).accounts({
            mint: mint.publicKey,
            adminAccount: adminAccount.publicKey,
            voterTokenAccount,
            votingTokenAccount,
            authority: user1.publicKey
        }).signers([user1]).rpc();
    });

    it("should perform vote from voter user2", async () => {
        const voterTokenAccount = await getAssociatedTokenAddress(mint.publicKey, user2.publicKey);
        const votingTokenAccount = await getAssociatedTokenAddress(mint.publicKey, votingAccount.publicKey);
        await program.methods.vote(votesAmount).accounts({
            mint: mint.publicKey,
            adminAccount: adminAccount.publicKey,
            voterTokenAccount,
            votingTokenAccount,
            authority: user2.publicKey
        }).signers([user2]).rpc();
    });

    it("should finish voting", async () => {
        setTimeout(async () => {
            const votingTokenAccount = await getAssociatedTokenAddress(mint.publicKey, votingAccount.publicKey);
            await program.methods.finishVoting().accounts({
                votingAccount: votingAccount.publicKey,
                votingTokenAccount,
                adminAccount: adminAccount.publicKey,
            }).rpc();
        }, 5);

    });
});
