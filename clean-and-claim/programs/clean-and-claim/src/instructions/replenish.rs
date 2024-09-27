use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, TokenInterface, TransferChecked, transfer_checked, Mint};

use crate::state::RewardAuthority;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Replenish<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"reward_authority", reward_authority.admin.as_ref(), reward_authority.mint.as_ref()],
        bump,
    )]
    pub reward_authority: Account<'info, RewardAuthority>,

    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = reward_authority,
        associated_token::token_program = token_program
    )]
    pub reward_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Replenish<'info> {
    pub fn replenish(ctx: Context<Replenish>, amount: u64) -> Result<()> {

        let authority = &ctx.accounts.reward_authority;

        if authority.only_admin_can_replenish {
            require!(ctx.accounts.user.key() == authority.admin, ErrorCode::OnlyAdminCanReplenish);
        }

        let cpi_accounts = TransferChecked {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.reward_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
            mint: ctx.accounts.reward_mint.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
        transfer_checked(cpi_ctx, amount, ctx.accounts.reward_mint.decimals)?;
        Ok(())
    }

    pub fn reclaim(ctx: Context<Replenish>) -> Result<()> {
        let authority = &ctx.accounts.reward_authority;

        // Check if the user is the admin
        require!(ctx.accounts.user.key() == authority.admin, ErrorCode::OnlyAdminCanReclaim);

        let amount = ctx.accounts.reward_token_account.amount;

        let admin = authority.admin.key();
        let mint = authority.mint.key();
        let signer_seeds = &[
            b"reward_authority",
            admin.as_ref(),
            mint.as_ref(),
        ];

        let cpi_accounts = TransferChecked {
            from: ctx.accounts.reward_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.reward_authority.to_account_info(),
            mint: ctx.accounts.reward_mint.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let binding = [&signer_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &binding);

        // Transfer all tokens from the reward token account back to the user
        transfer_checked(cpi_ctx, amount, ctx.accounts.reward_mint.decimals)?;
        Ok(())
    }
}

