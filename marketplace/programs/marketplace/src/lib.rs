use anchor_lang::prelude::*;

declare_id!("6F5Y4LTAtZSBmmeXHeQsQvwwJfdMLRCb8H4c2SZw3JTq");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
