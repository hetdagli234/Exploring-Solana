use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::state::reward_authority::RewardAuthority;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub rent_collector: Option<AccountInfo<'info>>,

    #[account(
        init,
        payer = admin,
        space = 8 + RewardAuthority::INIT_SPACE,
        seeds = [b"reward_authority", admin.key().as_ref(), reward_mint.key().as_ref()],
        bump,
    )]
    pub reward_authority: Account<'info, RewardAuthority>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = reward_mint,
        associated_token::authority = reward_authority,
        associated_token::token_program = token_program
    )]
    pub reward_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mint::token_program = token_program
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(ctx: Context<Initialize>, ratio: u64, only_admin_can_replenish: bool) -> Result<()> {
        ctx.accounts.reward_authority.set_inner(RewardAuthority {
            ratio,
            only_admin_can_replenish,
            admin: ctx.accounts.admin.key(),
            mint: ctx.accounts.reward_mint.key(),
            rent_collector: ctx.accounts.rent_collector.clone().unwrap_or(ctx.accounts.admin.to_account_info()).key(),
        });
    Ok(())
    }
}