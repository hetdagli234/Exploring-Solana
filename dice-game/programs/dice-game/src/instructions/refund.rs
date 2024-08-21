use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::state::Bet;

#[derive(Accounts)]
pub struct RefundBet<'info> {  
    #[account(mut)]
    pub player: Signer<'info>,
    /// CHECK: this is fine
    pub house: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault".as_ref(), house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"bet".as_ref(), vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>,
}

impl<'info> RefundBet<'info> {
    pub fn refund_bet(&mut self, bumps: &RefundBetBumps) -> Result<()> {
    let accounts = Transfer {
        from: self.vault.to_account_info(),
        to: self.player.to_account_info(),
    };

    let seeds = [b"vault", &self.house.key().to_bytes()[..], &[bumps.vault]];
    let signer_seeds = &[&seeds[..]][..];

    let ctx = CpiContext::new_with_signer(
        self.system_program.to_account_info(),
        accounts,
        signer_seeds
    );

    transfer(ctx, self.bet.amount)?;

    Ok(())
    }
}