use pinocchio::error::ProgramError;

mod check_contributions;
mod contribute;
mod initialize;
mod refund;

pub use check_contributions::*;
pub use contribute::*;
pub use initialize::*;
pub use refund::*;

/// Minimum target a fundraiser may set, before scaling by the mint's decimals.
pub const MIN_AMOUNT_TO_RAISE: u64 = 3;
/// Number of seconds in a day, used to convert the campaign duration to days.
pub const SECONDS_TO_DAYS: i64 = 86_400;
/// A single contributor may supply at most this percentage of the target.
pub const MAX_CONTRIBUTION_PERCENTAGE: u64 = 10;
/// Denominator for the percentage math above.
pub const PERCENTAGE_SCALER: u64 = 100;

/// Reads a little-endian `u64` starting at `offset` within `data`.
pub(crate) fn read_u64(data: &[u8], offset: usize) -> Result<u64, ProgramError> {
    let bytes: [u8; 8] = data
        .get(offset..offset + 8)
        .ok_or(ProgramError::InvalidInstructionData)?
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    Ok(u64::from_le_bytes(bytes))
}

/// Maximum amount a single contributor may supply: `MAX_CONTRIBUTION_PERCENTAGE`
/// percent of the campaign target.
pub(crate) fn max_contribution(amount_to_raise: u64) -> u64 {
    amount_to_raise
        .saturating_mul(MAX_CONTRIBUTION_PERCENTAGE)
        .saturating_div(PERCENTAGE_SCALER)
}
