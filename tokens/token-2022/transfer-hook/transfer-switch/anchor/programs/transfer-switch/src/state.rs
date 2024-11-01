use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TransferSwitch {
    pub wallet: Pubkey,
    pub on: bool,
}

#[account]
#[derive(InitSpace)]
pub struct AdminConfig {
    pub is_initialised: bool,
    pub admin: Pubkey,
}
