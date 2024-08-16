use anchor_lang::{prelude::*, solana_program::stake::state::Stake};
use anchor_spl::token::{Mint, Token};
use mpl_core::accounts::{BaseAssetV1, BaseCollectionV1};

use crate::state::config::StakeConfig;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub update_authority: Signer<'info>,
    #[account(
        init_if_needed, 
        payer = admin, 
        space = 8 + StakeConfig::INIT_SPACE, 
        seeds = [b"config".as_ref(), update_authority.key().as_ref(), seed.to_le_bytes().as_ref()], 
        bump,
    )]
    pub config: Account<'info, StakeConfig>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn init_config(&mut self, seed: u64) -> Result<()> {
        self.config.set_inner(StakeConfig {
            update_authority: self.update_authority.key(),
            seed,
        });
        Ok(())
    }
}