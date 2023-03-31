use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

pub fn __initialize_account(_ctx: Context<InitializeAccount>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = authority)]
    pub token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
