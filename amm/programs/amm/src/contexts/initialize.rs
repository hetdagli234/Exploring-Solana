use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenInterface, Mint, TransferChecked, transfer_checked};

use state::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    mint_x: InterfaceAccount<'info, Mint>,
    mint_y: InterfaceAccount<'info, Mint>,  
    #[account(
        init,
        payer = maker,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"amm", mint_x.key().as_ref(), mint_y.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    config: Account<'info, Config>,
    #[account(
        init_if_needed,
        payer = maker,
        mint::authority = config,
        mint::decimals = 6,
        mint::token_program = token_program,
        seeds = [b"mint", config.key().as_ref()],
        bump,
    )]
    mint_lp: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::authority = maker,
        associated_token::mint = mint_x,
        associated_token::token_program = token_program,
    )]
    vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::authority = maker,
        associated_token::mint = mint_y,
        associated_token::token_program = token_program,
    )]
    vault_y: InterfaceAccount<'info, TokenAccount>,  
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::authority = maker,
        associated_token::mint = mint_x,
        associated_token::token_program = token_program,
    )]
    maker_ata_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::authority = maker,
        associated_token::mint = mint_y,
        associated_token::token_program = token_program,
    )]
    maker_ata_y: InterfaceAccount<'info, TokenAccount>,  
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::authority = maker,
        associated_token::mint = mint_lp,
        associated_token::token_program = token_program,
    )]
    maker_ata_lp: InterfaceAccount<'info, TokenAccount>, 
    associated_token_program: Program<'info, TokenInterface>,
    token_program: Program<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn save_config(&mut self, seed: u64, fees: u16, bump: u8, mint_bump:u8) -> Result<()> {
        self.config.set_inner(Config {
            seed,
            fees,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            lp_bump,
            mint_bump,
        });
        Ok(())
    }
    pub fn deposit(&mut self, amount: u64, is_x: bool) -> Result<()> {
        let (from, to, mint) = match is_x {
            true => (self.maker_ata_x.to_account_info(), self.vault_x.to_account_info(), self.mint_x),
            false => (self.maker_ata_y.to_account_info(), self.vault_y.to_account_info(), self.mint_y),
        };

        let accounts  = TransferChecked {
            from,
            to,
            mint,
            authority: self.maker.to_account_info(),
        }
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        transfer_checked(ctx, amount, 6, mint, authority)
    }

    pub fn mint_lp_token(&mut self, amount_x: u64, amount_y: u64) -> Result<()> {
        let amount = amount_x.checked_mul(amount_y).ok_or(ProgramError::ArithmeticOverflow)?;

        let seed = &self.config.seed.to_le_bytes();
        let bump = self.config.bump;

        let signer_seeds = [&
        [b"amm"], 
        self.mint_x.to_account_info().key.as_ref()
        self.mint_y.to_account_info().key.as_ref() 
        self.config.seed.to_le_bytes(),
        ];
        
        let accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.maker_ata_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        mint_to(ctx, amount, 6, self.mint_lp, self.maker)
    }
}