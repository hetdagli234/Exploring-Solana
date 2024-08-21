use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::state::Bet;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    #[account(
        mut,
        seeds = ["vault".as_bytes(), house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, amount: u64) -> Result<()> {
        let account = Transfer {
            from: self.house.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), account);

        transfer(ctx, amount)?;

        Ok(())
    }

}