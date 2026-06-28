//! Shared fundraiser parameters, mirroring the native/Anchor example.

/// Minimum target amount a fundraiser may set, scaled by the mint's decimals:
/// the effective minimum is `MIN_AMOUNT_TO_RAISE.pow(decimals)`.
pub const MIN_AMOUNT_TO_RAISE: u64 = 3;

/// Seconds in a day, used to convert the elapsed time into whole days.
pub const SECONDS_TO_DAYS: i64 = 86400;

/// Largest share of the target a single contributor may provide, as a percent.
pub const MAX_CONTRIBUTION_PERCENTAGE: u64 = 10;

/// Denominator used together with [`MAX_CONTRIBUTION_PERCENTAGE`].
pub const PERCENTAGE_SCALER: u64 = 100;
