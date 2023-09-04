use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::RentVault;

pub fn init_rent_vault(ctx: Context<InitRentVault>, fund_lamports: u64) -> Result<()> {
    ctx.accounts.rent_vault.set_inner(RentVault {
        bump: *ctx.bumps.get(RentVault::SEED_PREFIX).unwrap(),
    });
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.rent_vault.to_account_info(),
            },
        ),
        fund_lamports,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct InitRentVault<'info> {
    #[account(
        init,
        space = RentVault::ACCOUNT_SPACE,
        payer = payer,
        seeds = [
            RentVault::SEED_PREFIX.as_bytes(),
        ],
        bump,
    )]
    rent_vault: Account<'info, RentVault>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}
