pub const DISCRIMINATOR_SIZE: usize = 8;
pub const STATE_SEED: &[u8] = b"state";
pub const WALLET_STATE_SEED: &[u8] = b"wallet_state";
pub const MINT_SEED: &[u8] = b"mint";

pub const ERROR_INSUFFICIENT_FUNDS: &str = "Insufficient funds for transfer";
pub const ERROR_INVALID_AUTHORITY: &str = "Invalid authority for operation";
pub const ERROR_WALLET_FROZEN: &str = "Wallet is frozen and cannot perform transfers";
pub const ERROR_INVALID_MINT: &str = "Invalid token mint";
pub const TOKEN_DECIMALS: u8 = 9;
pub const INITIAL_MINT_AMOUNT: u64 = 1_000_000_000;
