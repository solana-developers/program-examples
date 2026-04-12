use {
    crate::state::Favorites,
    quasar_lang::prelude::*,
};

/// Accounts for setting user favourites. Uses `init_if_needed` so the same
/// instruction can create or update the favourites PDA.
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: &'info mut Signer,
    #[account(mut, init_if_needed, payer = user, seeds = [b"favorites", user], bump)]
    pub favorites: Account<Favorites<'info>>,
    pub system_program: &'info Program<System>,
}

impl<'info> SetFavorites<'info> {
    #[inline(always)]
    pub fn set_favorites(&mut self, number: u64, color: &str) -> Result<(), ProgramError> {
        self.favorites.set_inner(
            number,
            color,
            self.user.to_account_view(),
            None,
        )
    }
}
