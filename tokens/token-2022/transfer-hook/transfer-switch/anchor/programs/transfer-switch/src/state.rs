use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TransferSwitch {
    pub on: bool,
}

#[account]
#[derive(InitSpace)]
pub struct AdminConfig {
    pub admin: Pubkey,
}
