use solana_program::msg;
use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum TutorialError {
    #[error("Invalid fee value")]
    InvalidFee = 0,

    #[error("Invalid mint for the pool")]
    InvalidMint = 1,

    #[error("Depositing too little liquidity")]
    DepositTooSmall = 2,

    #[error("Output is below the minimum expected")]
    OutputTooSmall = 3,

    #[error("Invariant does not hold")]
    InvariantViolated = 4,

    #[error("Account Validation breached")]
    ValidationBreached = 5,
}

impl TutorialError {
    pub fn print(&self) {
        match self {
            TutorialError::InvalidFee => msg!("Error: Invalid fee value"),
            TutorialError::InvalidMint => msg!("Error: Invalid mint for the pool"),
            TutorialError::DepositTooSmall => msg!("Error: Depositing too little liquidity"),
            TutorialError::OutputTooSmall => msg!("Error: Output is below the minimum expected"),
            TutorialError::InvariantViolated => msg!("Error: Invariant does not hold"),
            TutorialError::ValidationBreached => msg!("Error: Account validation breached"),
        }
    }
}

error!(TutorialError);
