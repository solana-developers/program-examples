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

impl<'info> TransferSolWithCpi<'info> {
    #[inline(always)]
    pub fn transfer_sol_with_cpi(&self, amount: u64) -> Result<(), ProgramError> {
        self.system_program
            .transfer(self.payer, self.recipient, amount)
            .invoke()
    }
}
