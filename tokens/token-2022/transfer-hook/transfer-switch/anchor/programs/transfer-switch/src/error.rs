use anchor_lang::prelude::*;

#[error_code]
pub enum TransferError {
    #[msg("The token is not currently transferring")]
    IsNotCurrentlyTransferring,

    #[msg("The transfer switch is currently not on")]
    SwitchNotOn,
}
