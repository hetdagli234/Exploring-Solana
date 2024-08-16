use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod error;

use instructions::*;
use state::*;

declare_id!("2kTv74CSrRnMdBjXhbq8dXnNFv4bCEuYMhTsjUeq5SvM");

#[program]
pub mod nft_staking_core {
    use super::*;

    pub fn init_config(ctx: Context<InitializeConfig>, seed: u64) -> Result<()> {
        ctx.accounts.init_config(seed)
    }

    pub fn stake(ctx: Context<CoreAsset>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn unstake(ctx: Context<CoreAsset>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
