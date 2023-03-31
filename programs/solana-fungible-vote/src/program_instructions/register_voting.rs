use anchor_lang::prelude::*;
use crate::program_accounts::{Voting, Admin};
use crate::program_instructions::error_codes::ErrorCodes;

pub fn __register_voting(ctx: Context<RegisterVoting>) -> Result<()> {
    let admin_account = &mut ctx.accounts.admin_account;
    require!(
        admin_account.current_voting.is_none(),
        ErrorCodes::VotingAlreadyStarted
    );
    let voting_account = &ctx.accounts.voting_account;
    admin_account.current_voting = Some(voting_account.to_account_info().key());
    Ok(())
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
