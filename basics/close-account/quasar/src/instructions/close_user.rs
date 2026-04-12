use {
    crate::state::UserState,
    quasar_lang::prelude::*,
};

/// Accounts for closing a user account.
/// The `close = user` attribute in the Anchor version triggers an automatic epilogue.
/// In Quasar, we call `close()` explicitly — it zeros the discriminator, drains lamports
/// to the destination, reassigns the owner to the system program, and resizes to 0.
#[derive(Accounts)]
pub struct CloseUser<'info> {
    #[account(mut)]
    pub user: &'info mut Signer,
    #[account(mut)]
    pub user_account: Account<UserState<'info>>,
}

#[inline(always)]
pub fn handle_close_user(accounts: &mut CloseUser) -> Result<(), ProgramError> {
    accounts.user_account.close(accounts.user.to_account_view())
}
