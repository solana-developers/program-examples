use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenInterface, Mint, TokenAccount, TransferChecked, CloseAccount, transfer_checked, close_account};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        has_one = mint_x,
        has_one = mint_y,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_x,
        associated_token::authority = taker,
    )]
    pub taker_mint_x_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_y,
        associated_token::authority = taker,
    )]
    pub taker_mint_y_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = maker,
    )]
    pub maker_mint_y_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = escrow.mint_x,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn take(&mut self) -> Result<()> {

        // Transfer amount from taker to maker
        self.transfer(self.escrow.amount, false)?;

        // Transfer amount from vault to taker
        self.transfer(self.vault.amount, true)
    }

    pub fn transfer(&mut self, amount: u64, is_x: bool) -> Result<()> {
        // Check if we are tranfering mint x or mint y
        match is_x {
            true => {
                // Create signer seeds
                let signer_seeds: [&[&[u8]];1] = [
                    &[
                        b"escrow", 
                        self.maker.to_account_info().key.as_ref(), 
                        &self.escrow.seed.to_le_bytes()[..],
                        &[self.escrow.bump]
                    ]
                ];

                // Create CPI accounts
                let cpi_accounts = TransferChecked {
                    from: self.vault.to_account_info(),
                    to: self.taker_mint_x_ata.to_account_info(),
                    authority: self.escrow.to_account_info(),
                    mint: self.mint_x.to_account_info(),
                };

                // Fetch CPI program
                let cpi_program = self.token_program.to_account_info();

                // Create CPI context
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

                // Transfer tokens
                transfer_checked(cpi_ctx, amount, self.mint_x.decimals)
            }

            false => {
                // Create CPI accounts
                let cpi_accounts = TransferChecked {
                    from: self.taker_mint_y_ata.to_account_info(),
                    to: self.maker_mint_y_ata.to_account_info(),
                    authority: self.taker.to_account_info(),
                    mint: self.mint_y.to_account_info(),
                };

                // Fetch CPI program
                let cpi_program = self.token_program.to_account_info();

                // Create CPI context
                let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

                // Transfer tokens
                transfer_checked(cpi_ctx, amount, self.mint_y.decimals)
            }
        }
    }

    pub fn close(&mut self) -> Result<()> {
        // Create signer seeds
        let signer_seeds: [&[&[u8]];1] = [
            &[
                b"escrow", 
                self.maker.to_account_info().key.as_ref(), 
                &self.escrow.seed.to_le_bytes()[..],
                &[self.escrow.bump]
            ]
        ];

        // Create CPI accounts
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        // Fetch CPI program
        let cpi_program = self.token_program.to_account_info();

        // Create CPI context
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        // Close vault
        close_account(cpi_ctx)
    }
}