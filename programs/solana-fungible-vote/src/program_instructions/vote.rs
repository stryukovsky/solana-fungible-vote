use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Transfer, transfer, Mint, TokenAccount};
use anchor_spl::associated_token::{get_associated_token_address};

use crate::program_accounts::Admin;
use crate::program_instructions::ErrorCodes;

pub fn __vote(ctx: Context<Vote>, amount: u64) -> Result<()> {        
    let admin_account = &ctx.accounts.admin_account;
    require!(admin_account.current_voting.is_some(), ErrorCodes::VotingUninitialized);
    let voting_address = admin_account.current_voting.unwrap();
    let token_mint_address = &ctx.accounts.mint.key();
    let voting_token_account = get_associated_token_address(&voting_address, token_mint_address);
    require!(voting_token_account.eq(&ctx.accounts.voting_token_account.key()), ErrorCodes::BadVotingAccount);
    let transfer_ctx = ctx.accounts.build_transfer_context();
    transfer(transfer_ctx, amount)
}


#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub voter_token_account: Account<'info, TokenAccount>,

    pub admin_account: Account<'info, Admin>,
    #[account(mut)]
    pub voting_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> Vote<'info> {
    pub fn build_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = Transfer{
            from: self.voter_token_account.to_account_info(),
            to: self.voting_token_account.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}
