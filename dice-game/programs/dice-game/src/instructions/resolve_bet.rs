use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use solana_program::{sysvar::instructions::load_instruction_at_checked, ed25519_program, hash::hash};

use crate::{state::Bet, errors::DiceError};

pub const HOUSE_EDGE:u16 = 150;

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    /// CHECK: kuch nahi hoga
    pub player: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = ["vault".as_bytes(), house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump,
    )]
    pub bet: Account<'info, Bet>,
    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    pub instruction_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {

    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;
        
        require_keys_eq!(ix.program_id, ed25519_program::ID, DiceError::Ed25519Program);

        require_eq!(ix.accounts.len(), 0, DiceError::Ed25519Accounts);

        let signature = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signature.len(), 1, DiceError::Ed25519DataLength);

        let signature = &signature[0];

        require_eq!(signature.is_verifiable, true, DiceError::Ed25519Header);

        require_keys_eq!(signature.public_key.ok_or(DiceError::Ed25519Pubkey)?, self.house.key(), DiceError::Ed25519Pubkey);

        require!(&signature.signature.ok_or(DiceError::Ed25519Signature)?.eq(sig), DiceError::Ed25519Signature);

        require!(&signature.message.as_ref().ok_or(DiceError::Ed25519Signature)?.eq(&self.bet.to_slice()), DiceError::Ed25519Signature);
        
        Ok(())
    }

    pub fn resolve_bet(&mut self, bumps: &ResolveBetBumps, sig: &[u8]) -> Result<()> {
        let hash = hash(sig).to_bytes();
        let mut hash_16:[u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[0..16]);

        let lower = u128::from_le_bytes(hash_16);
        hash_16.copy_from_slice(&hash[16..32]);

        let upper = u128::from_le_bytes(hash_16);

        let roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        if self.bet.roll > roll {
            let payment = (self.bet.amount as u128)
            .checked_mul(10000 - HOUSE_EDGE as u128).ok_or(DiceError::Overflow)?
            .checked_div(self.bet.roll as u128 - 1).ok_or(DiceError::Overflow)?
            .checked_div(100).ok_or(DiceError::Overflow)? as u64;
            
            let accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info()
            };

            let seeds = [b"vault", &self.house.key().to_bytes()[..], &[bumps.vault]];
            let signer_seeds = &[&seeds[..]][..];
    
            let ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                accounts,
                signer_seeds
            );
            transfer(ctx, payment)?;
        }

        Ok(())
    }
}
