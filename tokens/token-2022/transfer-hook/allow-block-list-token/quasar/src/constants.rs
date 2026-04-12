pub const META_LIST_ACCOUNT_SEED: &[u8] = b"extra-account-metas";
pub const CONFIG_SEED: &[u8] = b"config";
pub const AB_WALLET_SEED: &[u8] = b"ab_wallet";

/// SHA-256("spl-transfer-hook-interface:execute")[:8]
pub const EXECUTE_DISCRIMINATOR: [u8; 8] = [105, 37, 101, 197, 75, 251, 102, 26];

/// Maximum lengths for metadata fields.
pub const MAX_NAME: usize = 32;
pub const MAX_SYMBOL: usize = 10;
pub const MAX_URI: usize = 128;

/// Maximum buffer size for Token-2022 metadata CPI instructions.
pub const MAX_META_IX: usize = 512;
