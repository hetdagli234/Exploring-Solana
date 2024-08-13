use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Max stake reached")]
    MaxStakeReached,
    #[msg("No stake found")]
    NoStake,
}