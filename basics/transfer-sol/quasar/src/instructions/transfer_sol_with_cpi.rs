use quasar_lang::prelude::*;

/// Accounts for transferring SOL via system program CPI.
#[derive(Accounts)]
pub struct TransferSolWithCpi<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    #[account(mut)]
    pub recipient: &'info UncheckedAccount,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_transfer_sol_with_cpi(accounts: &TransferSolWithCpi, amount: u64) -> Result<(), ProgramError> {
    accounts.system_program
        .transfer(accounts.payer, accounts.recipient, amount)
        .invoke()
}
