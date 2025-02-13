use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum EscrowErrorCode {
    #[msg("Current SOL price is not above Escrow unlock price.")]
    SolPriceBelowUnlockPrice,
    #[msg("Feed account is not closed, must be closed to redeem with the withdraw_closed_feed_funds instruction.")]
    FeedAccountIsNotClosed,
    #[msg("Invalid withdrawal request")]
    InvalidWithdrawalRequest,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
#[error_code]
#[derive(Eq, PartialEq)]
pub enum SignatureVerificationError {
    #[msg("Signature not verified")]
    NotSigVerified,
    #[msg("Invalid signature data")]
    InvalidSignatureData,
    #[msg("Invalid Data  format")]
    InvalidDataFormat,
    #[msg("Less data than expected")]
    LessDataThanExpected,
    #[msg("Epoch too large")]
    EpochTooLarge,
}
