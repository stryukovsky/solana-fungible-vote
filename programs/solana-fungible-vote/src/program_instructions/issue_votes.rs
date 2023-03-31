use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, Mint, TokenAccount, Token, MintTo};

pub fn __issue_votes(ctx: Context<IssueVotes>, amount: u64) -> Result<()> {
    let mint_context = ctx.accounts.build_mint_context();
    mint_to(mint_context, amount)
}


#[derive(Accounts)]
pub struct IssueVotes<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> IssueVotes<'info> {
    pub fn build_mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = MintTo{
            mint: self.mint.to_account_info(),
            to: self.recipient.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}
