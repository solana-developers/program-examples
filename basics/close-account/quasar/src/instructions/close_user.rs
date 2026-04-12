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

impl<'info> CloseUser<'info> {
    #[inline(always)]
    pub fn close_user(&mut self) -> Result<(), ProgramError> {
        self.user_account.close(self.user.to_account_view())
    }
}
