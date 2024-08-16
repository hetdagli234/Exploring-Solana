use anchor_lang::prelude::*;

#[account()]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub points: i64,
    pub last_updated: i64,
    pub asset: Pubkey,
}