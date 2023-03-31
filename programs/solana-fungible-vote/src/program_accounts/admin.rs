use anchor_lang::prelude::*;

#[account]
pub struct Admin {
    pub authority: Pubkey,
    pub initialized_at: i64,
    pub current_voting: Option<Pubkey>,
}
