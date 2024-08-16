use anchor_lang::prelude::*;
use mpl_core::accounts::BaseAssetV1;
use mpl_core::instructions::{AddPluginV1CpiBuilder, RemovePluginV1CpiBuilder, UpdatePluginV1CpiBuilder};

use crate::state::stake_account::StakeAccount;
use crate::state::config::StakeConfig;
use crate::error::CustomError::*;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub update_authority: Account<'info, AccountInfo>,
    #[account(
        mut,
        has_one = owner,
        constraint = asset.update_authority == UpdateAuthority::Collection(collection.key()) @ InvalidUpdateAuthority,
    )]
    pub asset: Account<'info, BaseAssetV1>,
    #[account(
        seeds = [b"config".as_ref(), update_authority.key().as_ref(), config.seed.to_le_bytes().as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [b"stake_account", user.key().as_ref(), config.key().as_ref()],
        bump
    )]
    pub user_stake_account: Account<'info, StakeAccount>,
    #[account(address = mpl_core::ID)]
    pub core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self) -> ProgramResult<()> {

        self.user_stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            points: 0,
            last_updated: Clock::get()?.unix_timestamp,
            asset: self.asset,
        });

        AddPluginV1CpiBuilder::new(
        &self.core_program.to_account_info())
        .asset(&self.asset.to_account_info())
        .payer(&self.owner.to_account_info())
        .authority(Some(&self.owner.to_account_info()))
        .plugin(Plugin::FreezeDelegate( FreezeDelegate{ frozen: true } ))
        .init_authority(PluginAuthority::UpdateAuthority)
        .invoke()?
    }

    fn unstake(&mut self) -> ProgramResult<()> {

        let time_elapsed = ((Clock::get()?.unix_timestamp - self.user_stake_account.last_updated) / 86400) as u32;
        self.user_stake_account.points += time_elapsed as u32 * self.config.points_per_stake as u32;
        self.user_stake_account.last_updated = 0;

        //Frozen to false
        UpdatePluginV1CpiBuilder::new(&self.core_program.to_account_info())
        .asset(&self.asset.to_account_info())
        .collection(Some(&self.asset.collection.to_account_info()))
        .payer(&self.owner.to_account_info())
        .authority(Some(&self.update_authority.to_account_info()))
        .system_program(&self.system_program.to_account_info())
        .plugin(Plugin::FreezeDelegate( FreezeDelegate { frozen: false } ))
        .invoke()?;
    }

}
