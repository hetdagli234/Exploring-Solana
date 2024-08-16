use anchor_lang::prelude::*;
use mpl_core::accounts::BaseAssetV1;
use mpl_core::instructions::{AddPluginV1CpiBuilder, UpdatePluginV1CpiBuilder};
use mpl_core::types::{FreezeDelegate, Plugin, PluginAuthority};

use crate::state::stake_account::StakeAccount;
use crate::state::config::StakeConfig;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: This is a stupid think to do.
    pub update_authority: AccountInfo<'info>,
    #[account(
        mut,
        has_one = owner,
    )]
    pub asset: Account<'info, BaseAssetV1>,
    #[account(
        seeds = [b"config".as_ref(), update_authority.key().as_ref()],
        bump
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [b"stake_account", owner.key().as_ref(), config.key().as_ref()],
        bump
    )]
    pub user_stake_account: Account<'info, StakeAccount>,
    #[account(address = mpl_core::ID)]
    /// CHECK: This is a stupid think to do.
    pub core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self) -> Result<()> {

        self.user_stake_account.set_inner(StakeAccount {
            owner: self.owner.key(),
            points: 0,
            last_updated: Clock::get()?.unix_timestamp,
            asset: self.asset.key(),
        });

        Ok(AddPluginV1CpiBuilder::new(
        &self.core_program.to_account_info())
        .asset(&self.asset.to_account_info())
        .payer(&self.owner.to_account_info())
        .authority(Some(&self.owner.to_account_info()))
        .system_program(&self.system_program.to_account_info())
        .plugin(Plugin::FreezeDelegate( FreezeDelegate{ frozen: true } ))
        .init_authority(PluginAuthority::UpdateAuthority)
        .invoke()?)
    }

    pub fn unstake(&mut self) -> Result<()> {

        let time_elapsed = (Clock::get()?.unix_timestamp - self.user_stake_account.last_updated) / 86400;
        self.user_stake_account.points += time_elapsed;
        self.user_stake_account.last_updated = 0;

        //Frozen to false
        Ok(UpdatePluginV1CpiBuilder::new(&self.core_program.to_account_info())
        .asset(&self.asset.to_account_info())
        .payer(&self.owner.to_account_info())
        .authority(Some(&self.owner.to_account_info()))
        .system_program(&self.system_program.to_account_info())
        .plugin(Plugin::FreezeDelegate( FreezeDelegate { frozen: false } ))
        .invoke()?)
    }

}