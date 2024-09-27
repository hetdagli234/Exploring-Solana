use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Too many token accounts. Maximum allowed is 25.")]
    TooManyTokenAccounts,
    #[msg("Insufficient funds in the reward token account")]
    InsufficientRewardFunds,
    #[msg("Invalid mint for token account")]
    InvalidMint,
    #[msg("Invalid reward authority")]
    InvalidRewardAuthority,
    #[msg("Invalid user authority")]
    InvalidUserAuthority,
    #[msg("Only admin can replenish")]
    OnlyAdminCanReplenish,
    #[msg("Only admin can reclaim")]
    OnlyAdminCanReclaim,
    #[msg("Invalid rent collector")]
    InvalidRentCollector,
}
