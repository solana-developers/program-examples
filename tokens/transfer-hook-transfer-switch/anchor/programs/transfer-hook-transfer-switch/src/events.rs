use anchor_lang::prelude::*;

#[event]
#[derive(InitSpace)]
pub struct WalletStateChanged {
    pub wallet: Pubkey,
    pub is_frozen: bool,
    pub mint: Pubkey,
}

#[event]
#[derive(InitSpace)]
pub struct TransferProcessed {
    pub source: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
    pub mint: Pubkey,
}