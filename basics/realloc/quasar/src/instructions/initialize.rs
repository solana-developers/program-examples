use {
    crate::state::MessageAccount,
    quasar_lang::prelude::*,
};

/// Accounts for initialising a new message account.
/// The message_account is a random keypair (not a PDA) — same as the Anchor version.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: &'info mut Signer,
    #[account(mut, init, payer = payer)]
    pub message_account: Account<MessageAccount<'info>>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize(accounts: &mut Initialize, message: &str) -> Result<(), ProgramError> {
    accounts.message_account.set_inner(
        message,
        accounts.payer.to_account_view(),
        None,
    )
}
