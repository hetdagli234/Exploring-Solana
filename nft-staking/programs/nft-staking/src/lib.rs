use anchor_lang::prelude::*;
pub mod state;
use state::*;
pub mod instructions;
use instructions::*;
pub mod error;
use error::*;


declare_id!("AaSYv2PYCMHzDsDRmxm6Bbe2k8FXnetb5oNnUu48YsvA");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
