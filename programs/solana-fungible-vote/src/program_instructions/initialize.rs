use anchor_lang::prelude::*;
use crate::program_accounts::Admin;
use anchor_spl::token::{Token, Mint};

pub fn __initialize(ctx: Context<Initialize>) -> Result<()> {
    let admin_account = &mut ctx.accounts.admin_account;
    admin_account.authority = ctx.accounts.authority.key();
    admin_account.initialized_at = Clock::get().unwrap().unix_timestamp;
    admin_account.current_voting = None;
    Ok(())
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
