use quasar_lang::prelude::*;

/// Accounts for funding the rent vault PDA.
/// Transfers lamports from the payer to the vault via system program CPI.
/// When lamports are sent to a new address, the system program creates
/// a system-owned account automatically.
#[derive(Accounts)]
pub struct InitRentVault<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    #[account(mut, seeds = [b"rent_vault"], bump)]
    pub rent_vault: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_init_rent_vault(accounts: &InitRentVault, fund_lamports: u64) -> Result<(), ProgramError> {
    accounts.system_program
        .transfer(accounts.payer, accounts.rent_vault, fund_lamports)
        .invoke()
}
