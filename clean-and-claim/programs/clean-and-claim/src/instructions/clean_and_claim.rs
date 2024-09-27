use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::{error::ErrorCode, state::reward_authority::RewardAuthority};

#[derive(Accounts)]
pub struct CleanAndClaim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"reward_authority", reward_authority.admin.as_ref(), reward_authority.mint.as_ref()],
        bump,
    )]
    pub reward_authority: Account<'info, RewardAuthority>,

    /// CHECK: This is the hardcoded address to receive rent
    #[account(mut, address = reward_authority.rent_collector @ ErrorCode::InvalidRentCollector)]
    pub rent_collector: AccountInfo<'info>,

    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = reward_authority,
        associated_token::token_program = token_program
    )]
    pub reward_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_reward_token_account: InterfaceAccount<'info, TokenAccount>,

    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn clean<'c: 'info, 'info>(
    ctx: &Context<'_, '_, 'c, 'info, CleanAndClaim<'info>>
) -> Result<u64> {
    let mut closed_accounts = 0;

    for token_account in ctx.remaining_accounts.iter() {
        let cpi_accounts = CloseAccount {
            account: token_account.to_account_info(),
            destination: ctx.accounts.rent_collector.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        close_account(cpi_ctx)?;

        closed_accounts += 1;
    }

    Ok(closed_accounts)
}

pub fn claim<'c: 'info, 'info>(
    ctx: &mut Context<'_, '_, 'c, 'info, CleanAndClaim<'info>>,
    closed_accounts: u64
) -> Result<()> {
    let authority = &ctx.accounts.reward_authority;
    let admin = authority.admin.key();
    let mint = authority.mint.key();
    let seeds = [
        b"reward_authority",
        &admin.to_bytes()[..],
        &mint.to_bytes()[..],
        &[ctx.bumps.reward_authority],
    ];
    let signer_seeds = &[&seeds[..]][..];

    msg!("bumps: {:?}", ctx.bumps.reward_authority);
    msg!("signer seeds: {:?}", signer_seeds);


    let reward_amount = closed_accounts * authority.ratio * 10u64.pow(ctx.accounts.reward_mint.decimals as u32);
    
    // Check reward token account
    if ctx.accounts.reward_token_account.amount < reward_amount {
        return Err(ErrorCode::InsufficientRewardFunds.into());
    }
    if ctx.accounts.reward_token_account.mint != ctx.accounts.reward_mint.key() {
        return Err(ErrorCode::InvalidMint.into());
    }
    if ctx.accounts.reward_token_account.owner != ctx.accounts.reward_authority.key() {
        return Err(ErrorCode::InvalidRewardAuthority.into());
    }

    // Check user reward token account
    if ctx.accounts.user_reward_token_account.mint != ctx.accounts.reward_mint.key() {
        return Err(ErrorCode::InvalidMint.into());
    }

    // Log account information
    msg!("Reward Token Account: {:?}", ctx.accounts.reward_token_account.key());
    msg!("User Reward Token Account: {:?}", ctx.accounts.user_reward_token_account.key());
    msg!("Reward Authority: {:?}", ctx.accounts.reward_authority.key());
    msg!("Reward Mint: {:?}", ctx.accounts.reward_mint.key());
    msg!("Token Program: {:?}", ctx.accounts.token_program.key());
    msg!("Reward Amount: {}", reward_amount);

    // Additional logging for debugging
    msg!("Reward Token Account Owner: {:?}", ctx.accounts.reward_token_account.owner);
    msg!("Reward Token Account Mint: {:?}", ctx.accounts.reward_token_account.mint);
    msg!("User Reward Token Account Owner: {:?}", ctx.accounts.user_reward_token_account.owner);
    msg!("User Reward Token Account Mint: {:?}", ctx.accounts.user_reward_token_account.mint);
    msg!("Reward Mint Decimals: {:?}", ctx.accounts.reward_mint.decimals);

    let cpi_accounts = Transfer {
        from: ctx.accounts.reward_token_account.to_account_info(),
        to: ctx.accounts.user_reward_token_account.to_account_info(),
    };

    let ctx = CpiContext::new_with_signer(ctx.accounts.system_program.to_account_info(), cpi_accounts, signer_seeds);

    msg!("Executing TransferChecked...");
    transfer(ctx, reward_amount)?;
    msg!("TransferChecked completed successfully");

    Ok(())
}