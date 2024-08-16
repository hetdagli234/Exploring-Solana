use anchor_lang::prelude::*;
use crate::state::config::StakeConfig;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub update_authority: Signer<'info>,
    #[account(
        init_if_needed, 
        payer = update_authority, 
        space = 8 + StakeConfig::INIT_SPACE, 
        seeds = [b"config".as_ref(), update_authority.key().as_ref()], 
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