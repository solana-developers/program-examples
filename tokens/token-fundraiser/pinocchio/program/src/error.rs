//! Program-specific errors, mirroring the Anchor example's `FundraiserError`.

use pinocchio::error::ProgramError;

/// Errors returned by the fundraiser program.
///
/// Each variant maps to a [`ProgramError::Custom`] code equal to its position
/// in this enum, matching the order of the Anchor `#[error_code]` enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FundraiserError {
    /// The target amount has not been met yet.
    TargetNotMet,
    /// The target amount has already been met.
    TargetMet,
    /// The contribution exceeds the per-contributor maximum.
    ContributionTooBig,
    /// The contribution is below the minimum.
    ContributionTooSmall,
    /// The contributor has reached their maximum total contribution.
    MaximumContributionsReached,
    /// The fundraiser has not ended yet.
    FundraiserNotEnded,
    /// The fundraiser has already ended.
    FundraiserEnded,
    /// The target amount is invalid (must be at least `3^decimals`).
    InvalidAmount,
}

impl From<FundraiserError> for ProgramError {
    fn from(e: FundraiserError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
