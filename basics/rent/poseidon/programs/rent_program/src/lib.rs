use anchor_lang::prelude::*;
declare_id!("EHjrAJo1Ld77gkq6Pp2ErQHcC6FghT8BEPebNve8bAvj");
#[program]
pub mod rent_program {
    use super::*;
}
#[account]
pub struct AddressData {
    pub owner: Pubkey,
    pub id: u64,
    pub zip_code: u64,
    pub account_bump: u8,
}
