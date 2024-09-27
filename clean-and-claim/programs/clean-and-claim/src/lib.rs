use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;

pub use instructions::*;
pub use state::*;
pub use error::ErrorCode;

declare_id!("C5tBGFF2h8s2432GyS5xG5Qf5QWo7KJmfmUT9R8a51bu");
#[program]
pub mod clean_and_claim {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, ratio: u64, only_admin_can_replenish: bool) -> Result<()> {
        Initialize::initialize(ctx, ratio, only_admin_can_replenish)?;
        Ok(())
    }

    pub fn replenish(ctx: Context<Replenish>, amount: u64) -> Result<()> {
        Replenish::replenish(ctx, amount)
    }

    pub fn reclaim(ctx: Context<Replenish>) -> Result<()> {
        Replenish::reclaim(ctx)
    }

    pub fn clean_and_claim<'c: 'info, 'info>(mut ctx: Context<'_, '_, 'c, 'info, CleanAndClaim<'info>>) -> Result<()> {
        let closed_accounts = clean_and_claim::clean(&mut ctx)?;
        clean_and_claim::claim(&mut ctx, closed_accounts)?;
        Ok(())
    }
}