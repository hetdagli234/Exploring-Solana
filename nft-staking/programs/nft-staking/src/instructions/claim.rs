use anchor_lang::prelude::*;

use crate::state::{UserAccount, StakeConfig, StakeAccount};
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    pub stake_account: Account<'info, StakeAccount>,
    #[account(
        mint::decimals = 6,
        mint::authority = config,
        mint::token_program = token_program,
    )]
    pub rewards_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    pub user_ata: Account<'info, TokenAccount>,
    pub config: Account<'info, StakeConfig>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {

        let time_elapsed = ((Clock::get()?.unix_timestamp - self.stake_account.last_updated) / 86400) as u32;
        self.user_account.points += time_elapsed as u32 * self.config.points_per_stake as u32;

        let config_seeds = &[b"config".as_ref(), &[self.config.bump]];

        let mint_accounts = MintTo {
            authority: self.config.to_account_info(),
            to: self.user_ata.to_account_info(),
            mint: self.rewards_mint.to_account_info(),
        };
        
        let binding = [&config_seeds[..]];
        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            mint_accounts,
            &binding,
        );
    
        mint_to(
            cpi_context,
            self.user_account.points as u64,
        )?;

        self.user_account.points = 0;
        self.stake_account.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }
}