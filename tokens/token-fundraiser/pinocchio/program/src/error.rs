use pinocchio::error::ProgramError;

/// Errors returned by the fundraiser program.
///
/// These mirror the named errors of the Anchor and native versions of this
/// example. Each variant is surfaced to clients as `ProgramError::Custom(n)`,
/// where `n` is the variant's discriminant below.
#[repr(u32)]
pub enum FundraiserError {
    /// The vault does not yet hold the target amount.
    TargetNotMet,
    /// The target has already been reached, so a refund is not allowed.
    TargetMet,
    /// The contribution exceeds the per-contributor maximum.
    ContributionTooBig,
    /// The contribution is below the minimum.
    ContributionTooSmall,
    /// The contributor has reached their maximum total contribution.
    MaximumContributionsReached,
    /// The fundraiser has not ended yet, so a refund is not allowed.
    FundraiserNotEnded,
    /// The fundraiser has already ended, so contributions are closed.
    FundraiserEnded,
    /// The requested target is below the minimum allowed amount.
    InvalidAmount,
    /// A provided account is not the expected PDA for the given seeds.
    InvalidSeeds,
    /// A provided mint does not match the one the fundraiser is raising.
    InvalidMint,
    /// The provided vault is not a token account owned by the fundraiser for
    /// the raised mint.
    InvalidVault,
}

impl From<FundraiserError> for ProgramError {
    fn from(error: FundraiserError) -> Self {
        ProgramError::Custom(error as u32)
    }
}
