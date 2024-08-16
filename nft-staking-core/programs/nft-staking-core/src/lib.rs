use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("2kTv74CSrRnMdBjXhbq8dXnNFv4bCEuYMhTsjUeq5SvM");

#[program]
pub mod nft_staking_core {
    use super::*;

    pub fn init_config(ctx: Context<InitializeConfig>, seed: u64) -> Result<()> {
        ctx.accounts.init_config(seed)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake()
    }

    pub fn unstake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.unstake()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
