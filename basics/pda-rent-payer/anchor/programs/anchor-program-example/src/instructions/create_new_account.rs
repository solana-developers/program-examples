use anchor_lang::prelude::*;

use crate::state::RentVault;

pub fn create_new_account(ctx: Context<CreateNewAccount>) -> Result<()> {
    // Assuming this account has no inner data (size 0)
    //
    let lamports_required_for_rent = (Rent::get()?).minimum_balance(0);

    **ctx
        .accounts
        .rent_vault
        .to_account_info()
        .lamports
        .borrow_mut() -= lamports_required_for_rent;
    **ctx
        .accounts
        .new_account
        .to_account_info()
        .lamports
        .borrow_mut() += lamports_required_for_rent;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateNewAccount<'info> {
    /// CHECK: Unchecked
    #[account(mut)]
    new_account: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [
            RentVault::SEED_PREFIX.as_bytes().as_ref(),
        ],
        bump = rent_vault.bump,
    )]
    rent_vault: Account<'info, RentVault>,
    system_program: Program<'info, System>,
}
