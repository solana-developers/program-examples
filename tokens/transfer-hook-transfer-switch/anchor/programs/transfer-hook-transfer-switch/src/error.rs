use anchor_lang::prelude::*;

#[error_code]
pub enum TransferHookError {
    #[msg(ERROR_INSUFFICIENT_FUNDS)]
    InsufficientFunds,
    
    #[msg(ERROR_INVALID_AUTHORITY)]
    InvalidAuthority,
    
    #[msg(ERROR_WALLET_FROZEN)]
    WalletFrozen,

    #[msg(ERROR_INVALID_MINT)]
    InvalidMint,
}