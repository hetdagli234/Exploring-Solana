use anchor_lang::prelude::*;


pub mod state;
pub mod instructions;
pub mod error;

use instructions::*;

declare_id!("J7jh3C4HkTjWd71DqMxaZLohLJezDEXkm78A73nYjhRe");
#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.init_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }


    pub fn initialize_user_account(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.init_user_account(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.unstake(&ctx.bumps)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }

}
