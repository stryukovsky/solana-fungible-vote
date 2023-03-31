use anchor_lang::prelude::*;

#[account]
pub struct Voting {
    pub started_by: Pubkey,
    pub quorum: u64,
    pub voting_until: i64,
    pub is_applied: Option<bool>,
}
