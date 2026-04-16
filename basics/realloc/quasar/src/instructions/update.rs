use {
    crate::state::MessageAccount,
    quasar_lang::prelude::*,
};

/// Accounts for updating a message account.
/// Quasar's `set_inner` automatically handles realloc when the new message
/// is longer than the current account data. No explicit realloc needed.
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub payer: &'info mut Signer,
    #[account(mut)]
    pub message_account: Account<MessageAccount<'info>>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_update(accounts: &mut Update, message: &str) -> Result<(), ProgramError> {
    accounts.message_account.set_inner(
        message,
        accounts.payer.to_account_view(),
        None,
    )
}
