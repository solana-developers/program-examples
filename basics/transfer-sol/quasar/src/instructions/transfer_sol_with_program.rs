use quasar_lang::prelude::*;

/// Accounts for transferring SOL by directly manipulating lamports.
/// The payer account must be owned by this program for direct lamport access.
#[derive(Accounts)]
pub struct TransferSolWithProgram<'info> {
    #[account(mut)]
    pub payer: &'info UncheckedAccount,
    #[account(mut)]
    pub recipient: &'info UncheckedAccount,
}

impl<'info> TransferSolWithProgram<'info> {
    #[inline(always)]
    pub fn transfer_sol_with_program(&self, amount: u64) -> Result<(), ProgramError> {
        let payer_view = self.payer.to_account_view();
        let recipient_view = self.recipient.to_account_view();
        set_lamports(payer_view, payer_view.lamports() - amount);
        set_lamports(recipient_view, recipient_view.lamports() + amount);
        Ok(())
    }
}
