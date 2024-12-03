use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TransferHookState {
    pub authority: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct WalletState {
    pub is_frozen: bool,
    pub bump: u8,
    pub mint: Pubkey,
    pub owner: Pubkey,
}
