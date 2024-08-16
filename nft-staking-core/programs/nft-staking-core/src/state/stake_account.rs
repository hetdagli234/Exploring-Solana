use anchor_lang::prelude::*;
use mpl_core::accounts::BaseAssetV1;

#[account()]
#[derive(InitSpace)]
pub struct StakeAccount {
    #[max_len(32)]
    pub owner: Pubkey,
    pub points: u128,
    pub last_updated: i64,
    #[max_len(32)]
    pub asset: BaseAssetV1,
}