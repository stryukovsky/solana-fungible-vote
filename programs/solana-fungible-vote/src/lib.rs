use anchor_lang::prelude::*;
pub mod program_accounts;
pub mod program_instructions;
pub use program_accounts::*;
pub use program_instructions::*;

declare_id!("LAdrLGGqMZdUh5CFpbeJcvfHgzEzZxWo3gLSUfp68Mn");

#[program]
pub mod solana_fungible_vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
       __initialize(ctx)
    }

    pub fn initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
        __initialize_account(ctx)
    }

    pub fn initialize_voting(ctx: Context<InitializeVoting>, quorum: u64, voting_until: i64) -> Result<()> {
        __initialize_voting(ctx, quorum, voting_until)
    }

    pub fn register_voting(ctx: Context<RegisterVoting>) -> Result<()> {
        __register_voting(ctx)
    }

    pub fn issue_votes(ctx: Context<IssueVotes>, amount: u64) -> Result<()> {
        __issue_votes(ctx, amount)
    }

    pub fn vote(ctx: Context<Vote>, amount: u64) -> Result<()> {
        __vote(ctx, amount)
    }

    pub fn finish_voting(ctx: Context<FinishVoting>) -> Result<()> {
        __finish_voting(ctx)
    }

 }
