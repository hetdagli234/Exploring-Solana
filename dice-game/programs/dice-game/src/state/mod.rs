use anchor_lang::prelude::*;

#[account]
pub struct Bet {
    pub player: Pubkey,
    pub seed: u64,
    pub amount: u64,
    pub roll: u8,
    pub slot: u64,
    pub bump: u8,
}

impl Bet {
    pub const LEN: usize = 8 + 32 + 8*3 + 1*2;

    pub fn to_slice(&self) -> Vec<u8> {
        let mut s = self.player.to_bytes().to_vec();
        s.extend_from_slice(&self.seed.to_le_bytes());
        s.extend_from_slice(&self.amount.to_le_bytes());
        s.extend_from_slice(&self.roll.to_le_bytes());
        s.extend_from_slice(&self.slot.to_le_bytes());
        s.extend_from_slice(&self.bump.to_le_bytes());

        s
    }
}