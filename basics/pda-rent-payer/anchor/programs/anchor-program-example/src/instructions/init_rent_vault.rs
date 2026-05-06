use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct InitRentVault<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"rent_vault",
        ],
        bump,
    )]
    rent_vault: SystemAccount<'info>,
    system_program: Program<'info, System>,
}

// When lamports are transferred to a new address (without and existing account),
// An account owned by the system program is created by default
pub fn handle_init_rent_vault(context: Context<InitRentVault>, fund_lamports: u64) -> Result<()> {
    transfer(
        CpiContext::new(
            context.accounts.system_program.key(),
            Transfer {
                from: context.accounts.payer.to_account_info(),
                to: context.accounts.rent_vault.to_account_info(),
            },
        ),
        fund_lamports,
    )?;
    Ok(())
}
