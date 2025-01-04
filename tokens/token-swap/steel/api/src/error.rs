use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum TokenSwapError {
    #[error("Invalid fee, must be between 0 and 10000")]
    InvalidFee = 0,
    #[error("Account is not existed")]
    AccountIsNotExisted = 1,
    #[error("Invalid account")]
    InvalidAccount = 2,
    #[error("Deposit too small")]
    DepositTooSmall = 3,
    #[error("Withdrawal too small")]
    OutputTooSmall,
    #[error("Invariant violated")]
    InvariantViolated,
}

error!(TokenSwapError);
