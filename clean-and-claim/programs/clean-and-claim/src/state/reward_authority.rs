use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct RewardAuthority {
    pub admin: Pubkey,
    pub ratio: u64,
    pub only_admin_can_replenish: bool,
    pub mint: Pubkey,
    pub rent_collector: Pubkey,
}