use anchor_lang::prelude::*;
use crate::program_accounts::{Voting, Admin};
use anchor_spl::token::{Token, TokenAccount};
use crate::program_instructions::ErrorCodes;

pub fn __finish_voting(ctx: Context<FinishVoting>) -> Result<()> {
    let voting_account = &mut ctx.accounts.voting_account;
    let quorum = voting_account.quorum;
    let voting_token_account = &ctx.accounts.voting_token_account;
    let current_balance = voting_token_account.amount;
    let current_time = Clock::get().unwrap().unix_timestamp;
    let end_time = voting_account.voting_until;
    require!(end_time <= current_time, ErrorCodes::VotingNotFinished);
    let admin_account = &mut ctx.accounts.admin_account;
    require!(admin_account.current_voting.is_some(), ErrorCodes::VotingUninitialized);
    require!(admin_account.current_voting.unwrap().eq(&voting_account.key()), ErrorCodes::BadVoting);
    voting_account.is_applied = Some(quorum <= current_balance);
    admin_account.current_voting = None;
    Ok(())
}

#[derive(Accounts)]
pub struct FinishVoting<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub voting_account: Account<'info, Voting>,

    #[account(mut)]
    voting_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub admin_account: Account<'info, Admin>,

    pub token_program: Program<'info, Token>,
}
