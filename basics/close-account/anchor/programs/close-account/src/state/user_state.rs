use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct UserState {
    pub bump: u8,     // 1 byte
    pub user: Pubkey, // 32 bytes
    #[max_len(50)] // set a max length for the string
    pub name: String, // 4 bytes + 50 bytes
}
