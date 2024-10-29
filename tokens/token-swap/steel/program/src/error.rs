use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum SteelError {
    #[error("Invalid fee value")]
    InvalidFee,

    #[error("Invalid mint for the pool")]
    InvalidMint,

    #[error("Depositing too little liquidity")]
    DepositTooSmall,

    #[error("Output is below the minimum expected")]
    OutputTooSmall,

    #[error("Invariant does not hold")]
    InvariantViolated,
}

error!(SteelError);
