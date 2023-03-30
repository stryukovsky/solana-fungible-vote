use anchor_lang::prelude::*;
use anchor_spl::token::{Token, MintTo, mint_to, transfer, Transfer, Mint, TokenAccount};
use anchor_spl::associated_token::{AssociatedToken};

declare_id!("LAdrLGGqMZdUh5CFpbeJcvfHgzEzZxWo3gLSUfp68Mn");

#[program]
pub mod solana_fungible_vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        admin_account.authority = ctx.accounts.authority.key();
        admin_account.initialized_at = Clock::get().unwrap().unix_timestamp;
        admin_account.current_voting = None;
        Ok(())
    }

    pub fn initialize_account(_ctx: Context<RegisterAccount>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_voting(ctx: Context<InitializeVoting>, quorum: u64, voting_until: i64) -> Result<()> {
        let admin_account = &ctx.accounts.admin_account;
        require!(admin_account.current_voting.is_none(), ErrorCodes::VotingAlreadyStarted);
        let voting_account = &mut ctx.accounts.voting_account;
        voting_account.started_by = ctx.accounts.authority.key();
        voting_account.voted_for = 0;
        voting_account.voted_against = 0;
        voting_account.quorum = quorum;
        voting_account.voting_until = voting_until;
        Ok(())
    }

    pub fn register_voting(ctx: Context<RegisterVoting>) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        require!(admin_account.current_voting.is_none(), ErrorCodes::VotingAlreadyStarted);
        let voting_account = &ctx.accounts.voting_account;
        admin_account.current_voting = Some(voting_account.to_account_info().key());
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>) -> Result<()> {
        let transfer_ctx = ctx.accounts.build_transfer_context();
        Ok(())
    }

 }

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, mint::authority = authority, mint::decimals = 6)]
    pub mint: Account<'info, Mint>,

    #[account(init, payer = authority, space = 8 + 32 + 8 + 1 + 32)]
    pub admin_account: Account<'info, Admin>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

}

#[error_code]
enum ErrorCodes {
    VotingAlreadyStarted,
}


#[derive(Accounts)]
pub struct RegisterAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = authority)]
    pub token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct InitializeVoting<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer=authority, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub voting_account: Account<'info, Voting>,

    pub admin_account: Account<'info, Admin>,

    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = voting_account)]
    pub token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct RegisterVoting<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub voting_account: Account<'info, Voting>,

    #[account(mut)]
    pub admin_account: Account<'info, Admin>,
}


#[account]
pub struct Admin {
    pub authority: Pubkey,
    pub initialized_at: i64,
    pub current_voting: Option<Pubkey>,
}

#[account]
pub struct Voting {
    pub started_by: Pubkey,
    pub quorum: u64,
    pub voted_for: u64,
    pub voted_against: u64,
    pub voting_until: i64,
}
