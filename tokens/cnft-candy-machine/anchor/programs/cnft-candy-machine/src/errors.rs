use anchor_lang::error_code;

#[error_code]
pub enum CustomError {
    #[msg("User Not Allowed")]
    UserNotAllowed,
    #[msg("User Already Claimed")]
    AlreadyClaimed,
    #[msg("Invalid Allow Mint")]
    InvalidAllowMint,
    #[msg("Invalid Allow Mint ATA")]
    InvalidAllowMintATA,
    #[msg("Candy Machine is not active")]
    CandyMachineInactive,
    #[msg("Inavalid SPL Settings")]
    InvalidSPLSettings,
    #[msg("Invalid number of remaining accounts")]
    InvalidRemainingAccounts,
    #[msg("Invalid source remaining accounts")]
    InvalidSourceRemainingAccount,
    #[msg("Invalid destination remaining accounts")]
    InvalidDestinationRemainingAccount,
}