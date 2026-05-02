use quasar_lang::prelude::Address;

/// On-chain layout for ABWallet: [32 bytes wallet] [1 byte allowed]
/// Total = 33 bytes.
pub const AB_WALLET_SIZE: u64 = 33;

/// On-chain layout for Config: [32 bytes authority] [1 byte bump]
/// Total = 33 bytes.
pub const CONFIG_SIZE: u64 = 33;

/// Mode discriminator values stored in Token-2022 metadata.
pub const MODE_ALLOW: &[u8] = b"Allow";
pub const MODE_BLOCK: &[u8] = b"Block";
pub const MODE_MIXED: &[u8] = b"Mixed";

/// Read wallet pubkey from ABWallet account data at offset 0.
pub fn read_wallet_address(data: &[u8]) -> &[u8; 32] {
    // Safety: caller must ensure data.len() >= 33
    data[0..32].try_into().unwrap()
}

/// Read the `allowed` flag from ABWallet account data.
pub fn read_wallet_allowed(data: &[u8]) -> bool {
    data[32] != 0
}

/// Read authority pubkey from Config account data at offset 0.
pub fn read_config_authority(data: &[u8]) -> &[u8; 32] {
    data[0..32].try_into().unwrap()
}

/// Read bump from Config account data at offset 32.
pub fn read_config_bump(data: &[u8]) -> u8 {
    data[32]
}

/// Mode enum for instruction arguments (serialized as a single byte).
/// 0 = Allow, 1 = Block, 2 = Mixed.
pub fn mode_to_metadata_value(mode_byte: u8) -> &'static [u8] {
    match mode_byte {
        0 => MODE_ALLOW,
        1 => MODE_BLOCK,
        2 => MODE_MIXED,
        _ => MODE_ALLOW,
    }
}

/// Convert a mode string from metadata back to the byte discriminator.
pub fn metadata_value_to_mode(value: &[u8]) -> u8 {
    if value == MODE_ALLOW {
        0
    } else if value == MODE_BLOCK {
        1
    } else if value == MODE_MIXED {
        2
    } else {
        0
    }
}

/// Write ABWallet data to account.
pub fn write_ab_wallet(data: &mut [u8], wallet: &Address, allowed: bool) {
    data[0..32].copy_from_slice(wallet.as_ref());
    data[32] = if allowed { 1 } else { 0 };
}

/// Write Config data to account.
pub fn write_config(data: &mut [u8], authority: &Address, bump: u8) {
    data[0..32].copy_from_slice(authority.as_ref());
    data[32] = bump;
}
