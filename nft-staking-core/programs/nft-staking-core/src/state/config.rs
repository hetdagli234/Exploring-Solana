use anchor_lang::prelude::*;


//Only one config per collection
#[account()]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub update_authority: Pubkey,
    pub seed: u64,
}