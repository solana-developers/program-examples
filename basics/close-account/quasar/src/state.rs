use quasar_lang::prelude::*;

/// User account with a dynamic name field.
/// Fixed fields (bump, user) must precede dynamic fields (name).
#[account(discriminator = 1)]
pub struct UserState<'a> {
    pub bump: u8,
    pub user: Address,
    pub name: String<u8, 50>,
}
