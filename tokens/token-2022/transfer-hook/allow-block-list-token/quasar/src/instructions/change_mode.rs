use quasar_lang::cpi::{BufCpiCall, InstructionAccount};
use quasar_lang::prelude::*;
use quasar_lang::sysvars::Sysvar;

use crate::constants::MAX_META_IX;
use crate::instructions::init_mint::Token2022;
use crate::state::mode_to_metadata_value;

#[derive(Accounts)]
pub struct ChangeMode<'info> {
    #[account(mut)]
    pub authority: &'info Signer,
    #[account(mut)]
    pub mint: &'info UncheckedAccount,
    pub token_program: &'info Program<Token2022>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_change_mode(accounts: &ChangeMode, mode: u8, threshold: u64) -> Result<(), ProgramError> {
    let mode_value = mode_to_metadata_value(mode);
    let token_prog = accounts.token_program.to_account_view().address();
    let mint_key = accounts.mint.to_account_view().address();
    let auth_key = accounts.authority.to_account_view().address();

    // Update "AB" metadata field
    emit_update_field(token_prog, mint_key, auth_key, accounts, b"AB", mode_value)?;

    // If Mixed mode or if metadata already has a threshold key, update/set threshold
    let is_mixed = mode == 2;
    let has_existing_threshold = has_threshold_in_metadata(accounts)?;

    if is_mixed || has_existing_threshold {
        let actual_threshold = if is_mixed { threshold } else { 0 };
        let mut threshold_buf = [0u8; 20];
        let threshold_len = write_u64_to_buf(actual_threshold, &mut threshold_buf);
        emit_update_field(
            token_prog,
            mint_key,
            auth_key,
            accounts,
            b"threshold",
            &threshold_buf[..threshold_len],
        )?;
    }

    // Top up mint rent if metadata grew
    let mint_view = accounts.mint.to_account_view();
    let data_len = mint_view.data_len();
    let min_balance = Rent::get()?.try_minimum_balance(data_len)?;
    let current_lamports = mint_view.lamports();
    if min_balance > current_lamports {
        let diff = min_balance - current_lamports;
        accounts.system_program
            .transfer(accounts.authority, &*accounts.mint, diff)
            .invoke()?;
    }

    log("Mode changed");
    Ok(())
}

/// Emit a TokenMetadataUpdateField CPI (opcode 44, sub-opcode 1).
fn emit_update_field(
    token_prog: &Address,
    mint_key: &Address,
    auth_key: &Address,
    ctx: &ChangeMode<'_>,
    key: &[u8],
    value: &[u8],
) -> Result<(), ProgramError> {
    let mut buf = [0u8; MAX_META_IX];
    let mut pos = 0;
    buf[pos] = 44;
    pos += 1;
    buf[pos] = 1; // UpdateField
    pos += 1;
    buf[pos] = 2; // Field::Key enum discriminator
    pos += 1;
    buf[pos..pos + 4].copy_from_slice(&(key.len() as u32).to_le_bytes());
    pos += 4;
    buf[pos..pos + key.len()].copy_from_slice(key);
    pos += key.len();
    buf[pos..pos + 4].copy_from_slice(&(value.len() as u32).to_le_bytes());
    pos += 4;
    buf[pos..pos + value.len()].copy_from_slice(value);
    pos += value.len();

    BufCpiCall::new(
        token_prog,
        [
            InstructionAccount::writable(mint_key),
            InstructionAccount::readonly_signer(auth_key),
        ],
        [
            ctx.mint.to_account_view(),
            ctx.authority.to_account_view(),
        ],
        buf,
        pos,
    )
    .invoke()
}

/// Check if the mint's metadata already contains a "threshold" key.
fn has_threshold_in_metadata(ctx: &ChangeMode<'_>) -> Result<bool, ProgramError> {
    let mint_view = ctx.mint.to_account_view();
    let data = mint_view.try_borrow()?;

    // Skip base mint (82) + padding (82) + account type (1) = 165
    let tlv_start = 165;
    if data.len() < tlv_start + 4 {
        return Ok(false);
    }

    let mut pos = tlv_start;
    while pos + 4 <= data.len() {
        let ext_type = u16::from_le_bytes([data[pos], data[pos + 1]]);
        let ext_len = u16::from_le_bytes([data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;

        if pos + ext_len > data.len() {
            break;
        }

        if ext_type == 18 {
            // TokenMetadata — parse additional_metadata
            let md = &data[pos..pos + ext_len];
            let mut mpos = 64; // skip update_authority + mint

            // Skip name, symbol, uri (borsh strings)
            for _ in 0..3 {
                if mpos + 4 > md.len() {
                    return Ok(false);
                }
                let slen = u32::from_le_bytes([md[mpos], md[mpos+1], md[mpos+2], md[mpos+3]]) as usize;
                mpos += 4 + slen;
            }

            // Read kv count
            if mpos + 4 > md.len() {
                return Ok(false);
            }
            let kv_count = u32::from_le_bytes([md[mpos], md[mpos+1], md[mpos+2], md[mpos+3]]) as usize;
            mpos += 4;

            for _ in 0..kv_count {
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

                if mpos + 4 > md.len() {
                    break;
                }
                let val_len = u32::from_le_bytes([md[mpos], md[mpos+1], md[mpos+2], md[mpos+3]]) as usize;
                mpos += 4 + val_len;

                if key == b"threshold" {
                    return Ok(true);
                }
            }
        }

        pos += ext_len;
    }

    Ok(false)
}

/// Write a u64 as decimal ASCII into a buffer. Returns the number of bytes written.
fn write_u64_to_buf(mut value: u64, buf: &mut [u8]) -> usize {
    if value == 0 {
        buf[0] = b'0';
        return 1;
    }
    let mut tmp = [0u8; 20];
    let mut pos = 0;
    while value > 0 {
        tmp[pos] = b'0' + (value % 10) as u8;
        value /= 10;
        pos += 1;
    }
    for i in 0..pos {
        buf[i] = tmp[pos - 1 - i];
    }
    pos
}
