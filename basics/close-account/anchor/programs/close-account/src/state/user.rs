use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub bump: u8,
    pub user: Pubkey,
    pub name: String,
}

impl User {
    pub const PREFIX: &'static str = "USER";

    pub const SIZE: usize = 8 + /* discriminator */
        std::mem::size_of::<u8>() + /* bump */
        std::mem::size_of::<Pubkey>() + /* user */
        80 /* name */
    ;
}
