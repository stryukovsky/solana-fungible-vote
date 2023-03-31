use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use crate::program_accounts::{Admin, Voting};
use crate::program_instructions::ErrorCodes;

pub fn __initialize_voting(ctx: Context<InitializeVoting>, quorum: u64, voting_until: i64) -> Result<()> {
    let admin_account = &ctx.accounts.admin_account;
    require!(
        admin_account.current_voting.is_none(),
        ErrorCodes::VotingAlreadyStarted
    );
    let voting_account = &mut ctx.accounts.voting_account;
    voting_account.started_by = ctx.accounts.authority.key();
    voting_account.quorum = quorum;
    voting_account.voting_until = voting_until;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVoting<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer=authority, space = 8 + 32 + 8 + 8 + 2)]
    pub voting_account: Account<'info, Voting>,

    pub admin_account: Account<'info, Admin>,

    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = voting_account)]
    pub voting_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
