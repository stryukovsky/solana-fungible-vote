use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCodes {
    VotingAlreadyStarted,
    VotingUninitialized,
    BadVotingAccount,
    VotingNotFinished,
    BadVoting,
}
