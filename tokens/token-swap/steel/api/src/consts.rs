pub const MINIMUM_LIQUIDITY: u64 = 100;

pub const AUTHORITY_SEED: &[u8] = b"authority";

pub const LIQUIDITY_SEED: &[u8] = b"liquidity";

/// The seed of the mint account PDA.
pub const MINT: &[u8] = b"mint";

/// Noise for deriving the mint pda
pub const MINT_NOISE: [u8; 16] = [
    89, 157, 88, 232, 243, 249, 197, 132, 199, 49, 19, 234, 91, 94, 150, 41,
];
