use const_crypto::ed25519;
use solana_program::pubkey::Pubkey;

/// Program id for const pda derivations
const PROGRAM_ID: [u8; 32] = unsafe { *(&crate::id() as *const Pubkey as *const [u8; 32]) };

/// The seed of the mint authority account PDA.
pub const MINT_AUTHORITY: &[u8] = b"mint_authority";

/// The seed of the mint account PDA.
pub const MINT: &[u8] = b"mint";

/// Noise for deriving the mint pda
pub const MINT_NOISE: [u8; 16] = [
    89, 157, 88, 232, 243, 249, 197, 132, 199, 49, 19, 234, 91, 94, 150, 41,
];

/// The seed of the metadata account PDA.
pub const METADATA: &[u8] = b"metadata";

/// The bump of the mint authority account, for cpis.
pub const MINT_AUTHORITY_BUMP: u8 =
    ed25519::derive_program_address(&[MINT_AUTHORITY], &PROGRAM_ID).1;
