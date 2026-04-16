use quasar_lang::prelude::*;

use crate::errors;
use crate::state::{read_wallet_allowed, MODE_ALLOW, MODE_BLOCK, MODE_MIXED, AB_WALLET_SIZE};

/// Transfer hook handler. Called by Token-2022 during transfers.
///
/// Account layout (fixed by the SPL transfer hook interface):
///   [0] source_token_account
///   [1] mint
///   [2] destination_token_account
///   [3] owner_delegate
///   [4] extra_account_meta_list
///   [5] ab_wallet — resolved from extra account metas (PDA for destination owner)
#[derive(Accounts)]
pub struct TxHook<'info> {
    pub source_token_account: &'info UncheckedAccount,
    pub mint: &'info UncheckedAccount,
    pub destination_token_account: &'info UncheckedAccount,
    pub owner_delegate: &'info UncheckedAccount,
    pub meta_list: &'info UncheckedAccount,
    pub ab_wallet: &'info UncheckedAccount,
}

#[inline(always)]
pub fn handle_tx_hook(accounts: &TxHook, amount: u64) -> Result<(), ProgramError> {
    let mint_view = accounts.mint.to_account_view();
    let mint_data = mint_view.try_borrow()?;

    let decoded_mode = decode_mint_mode(&mint_data)?;
    let decoded_wallet = decode_wallet_mode(accounts)?;

    match (decoded_mode, decoded_wallet) {
        // Allow mode: wallet must be on the allow list
        (DecodedMintMode::Allow, DecodedWalletMode::Allow) => Ok(()),
        (DecodedMintMode::Allow, _) => Err(errors::wallet_not_allowed()),

        // Any mode: blocked wallet is always blocked
        (_, DecodedWalletMode::Block) => Err(errors::wallet_blocked()),

        // Block mode: wallet is not blocked, so allow
        (DecodedMintMode::Block, _) => Ok(()),

        // Mixed/Threshold mode: check amount threshold
        (DecodedMintMode::Threshold(threshold), DecodedWalletMode::None)
            if amount >= threshold =>
        {
            Err(errors::amount_not_allowed())
        }
        (DecodedMintMode::Threshold(_), _) => Ok(()),
    }
}

fn decode_wallet_mode(accounts: &TxHook) -> Result<DecodedWalletMode, ProgramError> {
    let wallet_view = accounts.ab_wallet.to_account_view();
    if wallet_view.data_len() == 0 {
        return Ok(DecodedWalletMode::None);
    }

    // ABWallet on-chain: [32 bytes wallet] [1 byte allowed]
    let data = wallet_view.try_borrow()?;
    if data.len() < AB_WALLET_SIZE as usize {
        return Ok(DecodedWalletMode::None);
    }

    if read_wallet_allowed(&data) {
        Ok(DecodedWalletMode::Allow)
    } else {
        Ok(DecodedWalletMode::Block)
    }
}

enum DecodedMintMode {
    Allow,
    Block,
    Threshold(u64),
}

enum DecodedWalletMode {
    Allow,
    Block,
    None,
}

/// Parse Token-2022 mint account data to extract the mode from embedded
/// metadata. The metadata is stored as a TLV extension within the mint
/// account.
///
/// Token-2022 mint layout:
///   [0..82]   base Mint state
///   [82..164] padding (copy of base)
///   [164]     AccountType (2 = Mint)
///   [165..]   TLV extensions
///
/// Each TLV entry: [2 bytes type LE] [2 bytes length LE] [data]
///
/// We look for type 18 (TokenMetadata) and parse the additional_metadata
/// key-value pairs.
fn decode_mint_mode(data: &[u8]) -> Result<DecodedMintMode, ProgramError> {
    // Skip base mint (82) + padding (82) + account type (1) = 165
    let tlv_start = 165;
    if data.len() < tlv_start + 4 {
        return Err(errors::invalid_metadata());
    }

    let mut pos = tlv_start;
    let mut mode: Option<&[u8]> = None;
    let mut threshold: u64 = 0;

    // Walk TLV extensions to find TokenMetadata (type 18)
    while pos + 4 <= data.len() {
        let ext_type = u16::from_le_bytes([data[pos], data[pos + 1]]);
        let ext_len = u16::from_le_bytes([data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;

        if pos + ext_len > data.len() {
            break;
        }

        if ext_type == 18 {
            // Found TokenMetadata extension. Parse it.
            // Layout:
            //   [32 bytes update_authority]
            //   [32 bytes mint]
            //   [4 + N bytes name (borsh string)]
            //   [4 + N bytes symbol]
            //   [4 + N bytes uri]
            //   [4 bytes additional_metadata count]
            //   [repeated: (4+N key, 4+N value)]
            let md = &data[pos..pos + ext_len];
            let mut mpos = 64; // skip update_authority + mint

            // Skip name
            if mpos + 4 > md.len() {
                return Err(errors::invalid_metadata());
            }
            let name_len = u32::from_le_bytes([md[mpos], md[mpos+1], md[mpos+2], md[mpos+3]]) as usize;
            mpos += 4 + name_len;

            // Skip symbol
            if mpos + 4 > md.len() {
                return Err(errors::invalid_metadata());
            }
            let sym_len = u32::from_le_bytes([md[mpos], md[mpos+1], md[mpos+2], md[mpos+3]]) as usize;
            mpos += 4 + sym_len;

            // Skip uri
            if mpos + 4 > md.len() {
                return Err(errors::invalid_metadata());
            }
            let uri_len = u32::from_le_bytes([md[mpos], md[mpos+1], md[mpos+2], md[mpos+3]]) as usize;
            mpos += 4 + uri_len;

            // Read additional_metadata count
            if mpos + 4 > md.len() {
                return Err(errors::invalid_metadata());
            }
            let kv_count = u32::from_le_bytes([md[mpos], md[mpos+1], md[mpos+2], md[mpos+3]]) as usize;
            mpos += 4;

            for _ in 0..kv_count {
                // Read key
                if mpos + 4 > md.len() {
                    break;
                }
                let key_len = u32::from_le_bytes([md[mpos], md[mpos+1], md[mpos+2], md[mpos+3]]) as usize;
                mpos += 4;
                if mpos + key_len > md.len() {
                    break;
                }
                let key = &md[mpos..mpos + key_len];
                mpos += key_len;

                // Read value
                if mpos + 4 > md.len() {
                    break;
                }
                let val_len = u32::from_le_bytes([md[mpos], md[mpos+1], md[mpos+2], md[mpos+3]]) as usize;
                mpos += 4;
                if mpos + val_len > md.len() {
                    break;
                }
                let value = &md[mpos..mpos + val_len];
                mpos += val_len;

                if key == b"AB" {
                    mode = Some(value);
                    if value == MODE_ALLOW {
                        return Ok(DecodedMintMode::Allow);
                    } else if value == MODE_BLOCK {
                        return Ok(DecodedMintMode::Block);
                    } else if value == MODE_MIXED && threshold > 0 {
                        return Ok(DecodedMintMode::Threshold(threshold));
                    }
                } else if key == b"threshold" {
                    threshold = parse_u64_from_bytes(value);
                    if threshold > 0 && matches!(mode, Some(m) if m == MODE_MIXED) {
                        return Ok(DecodedMintMode::Threshold(threshold));
                    }
                }
            }
        }

        pos += ext_len;
    }

    // Fallback based on what we found
    match mode {
        Some(m) if m == MODE_ALLOW => Ok(DecodedMintMode::Allow),
        Some(m) if m == MODE_BLOCK => Ok(DecodedMintMode::Block),
        Some(m) if m == MODE_MIXED => Ok(DecodedMintMode::Threshold(threshold)),
        _ => Ok(DecodedMintMode::Allow), // default to Allow if no metadata found
    }
}

/// Parse a u64 from ASCII decimal bytes.
fn parse_u64_from_bytes(bytes: &[u8]) -> u64 {
    let mut result: u64 = 0;
    for &byte in bytes {
        if byte < b'0' || byte > b'9' {
            return 0;
        }
        result = result.saturating_mul(10).saturating_add((byte - b'0') as u64);
    }
    result
}
