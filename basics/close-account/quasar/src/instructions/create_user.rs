use {
    crate::state::UserState,
    quasar_lang::prelude::*,
};

/// Accounts for creating a new user.
#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub user: &'info mut Signer,
    #[account(mut, init, payer = user, seeds = [b"USER", user], bump)]
    pub user_account: Account<UserState<'info>>,
    pub system_program: &'info Program<System>,
}

impl<'info> CreateUser<'info> {
    #[inline(always)]
    pub fn create_user(&mut self, name: &str, bump: u8) -> Result<(), ProgramError> {
        let user_address = *self.user.to_account_view().address();
        self.user_account.set_inner(
            bump,
            user_address,
            name,
            self.user.to_account_view(),
            None,
        )
    }
}
