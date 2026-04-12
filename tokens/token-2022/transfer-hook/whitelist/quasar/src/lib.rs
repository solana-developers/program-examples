#![cfg_attr(not(test), no_std)]

use quasar_lang::sysvars::Sysvar;
use quasar_lang::{
    cpi::Seed,
    prelude::*,
};

#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

/// SPL Transfer Hook Interface discriminators (SHA-256 prefix).
#[allow(dead_code)]
const EXECUTE_DISCRIMINATOR: [u8; 8] = [105, 37, 101, 197, 75, 251, 102, 26];

/// Transfer hook that only allows transfers to whitelisted destination token
/// accounts. The whitelist is stored in a PDA seeded by ["white_list"].
#[program]
mod quasar_transfer_hook_whitelist {
    use super::*;

    /// Create the ExtraAccountMetaList PDA and the whitelist PDA.
    /// Discriminator = sha256("spl-transfer-hook-interface:initialize-extra-account-metas")[:8]
    #[instruction(discriminator = [43, 34, 13, 49, 167, 88, 235, 235])]
    pub fn initialize_extra_account_meta_list(
        ctx: Ctx<InitializeExtraAccountMetaList>,
    ) -> Result<(), ProgramError> {
        handle_initialize(&mut ctx.accounts)
    }

    /// Transfer hook handler — checks if the destination is in the whitelist.
    /// Discriminator = sha256("spl-transfer-hook-interface:execute")[:8]
    #[instruction(discriminator = [105, 37, 101, 197, 75, 251, 102, 26])]
    pub fn transfer_hook(ctx: Ctx<TransferHook>, _amount: u64) -> Result<(), ProgramError> {
        handle_transfer_hook(&mut ctx.accounts)
    }

    /// Add an address to the whitelist. Only callable by the authority.
    #[instruction(discriminator = [0, 0, 0, 0, 0, 0, 0, 2])]
    pub fn add_to_whitelist(ctx: Ctx<AddToWhitelist>) -> Result<(), ProgramError> {
        handle_add_to_whitelist(&mut ctx.accounts)
    }
}

// ---------------------------------------------------------------------------
// InitializeExtraAccountMetaList
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    #[account(mut)]
    pub extra_account_meta_list: &'info mut UncheckedAccount,
    pub mint: &'info UncheckedAccount,
    /// Whitelist PDA: ["white_list"]
    #[account(mut)]
    pub white_list: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize(accounts: &InitializeExtraAccountMetaList) -> Result<(), ProgramError> {
    // Create ExtraAccountMetaList PDA (1 extra account: whitelist)
    let meta_list_size: u64 = 51; // 8 + 4 + 4 + 35
    let lamports = Rent::get()?.try_minimum_balance(meta_list_size as usize)?;

    let mint_address = accounts.mint.to_account_view().address();
    let (expected_pda, bump) = Address::find_program_address(
        &[b"extra-account-metas", mint_address.as_ref()],
        &crate::ID,
    );

    if accounts.extra_account_meta_list.to_account_view().address() != &expected_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    let bump_bytes = [bump];
    let seeds = [
        Seed::from(b"extra-account-metas" as &[u8]),
        Seed::from(mint_address.as_ref()),
        Seed::from(&bump_bytes as &[u8]),
    ];

    accounts.system_program
        .create_account(accounts.payer, &*accounts.extra_account_meta_list, lamports, meta_list_size, &crate::ID)
        .invoke_signed(&seeds)?;

    // Write TLV data
    let view = unsafe {
        &mut *(accounts.extra_account_meta_list as *const UncheckedAccount
            as *mut UncheckedAccount as *mut AccountView)
    };
    let mut data = view.try_borrow_mut()?;
    data[0..8].copy_from_slice(&EXECUTE_DISCRIMINATOR);
    data[8..12].copy_from_slice(&39u32.to_le_bytes());
    data[12..16].copy_from_slice(&1u32.to_le_bytes());

    // ExtraAccountMeta for whitelist PDA: seeds = [Literal("white_list")]
    data[16] = 1; // PDA from seeds
    let mut config = [0u8; 32];
    config[0] = 1; // 1 seed
    config[1] = 0; // literal
    config[2] = 10; // length
    config[3..13].copy_from_slice(b"white_list");
    data[17..49].copy_from_slice(&config);
    data[49] = 0; // not signer
    data[50] = 1; // writable

    // Create whitelist PDA
    // Layout: [32 bytes authority] [4 bytes count] [N * 32 bytes addresses]
    // Allocate 400 bytes (enough for ~11 addresses)
    let wl_size: u64 = 400;
    let wl_lamports = Rent::get()?.try_minimum_balance(wl_size as usize)?;

    let (wl_pda, wl_bump) = Address::find_program_address(&[b"white_list"], &crate::ID);
    if accounts.white_list.to_account_view().address() != &wl_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    let wl_bump_bytes = [wl_bump];
    let wl_seeds = [
        Seed::from(b"white_list" as &[u8]),
        Seed::from(&wl_bump_bytes as &[u8]),
    ];

    accounts.system_program
        .create_account(accounts.payer, &*accounts.white_list, wl_lamports, wl_size, &crate::ID)
        .invoke_signed(&wl_seeds)?;

    // Write authority (payer) to whitelist account
    let wl_view = unsafe {
        &mut *(accounts.white_list as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut wl_data = wl_view.try_borrow_mut()?;
    wl_data[0..32].copy_from_slice(accounts.payer.to_account_view().address().as_ref());
    // count = 0 (already zeroed)

    log("Whitelist transfer hook initialized");
    Ok(())
}

// ---------------------------------------------------------------------------
// TransferHook
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct TransferHook<'info> {
    pub source_token: &'info UncheckedAccount,
    pub mint: &'info UncheckedAccount,
    pub destination_token: &'info UncheckedAccount,
    pub owner: &'info UncheckedAccount,
    pub extra_account_meta_list: &'info UncheckedAccount,
    pub white_list: &'info UncheckedAccount,
}

#[inline(always)]
pub fn handle_transfer_hook(accounts: &TransferHook) -> Result<(), ProgramError> {
    let wl_view = accounts.white_list.to_account_view();
    let data = wl_view.try_borrow()?;

    if data.len() < 36 {
        return Err(ProgramError::AccountDataTooSmall);
    }

    // Read count at offset 32
    let mut count_bytes = [0u8; 4];
    count_bytes.copy_from_slice(&data[32..36]);
    let count = u32::from_le_bytes(count_bytes) as usize;

    // Check if destination is in the whitelist
    let dest_address = accounts.destination_token.to_account_view().address();
    let mut found = false;
    for i in 0..count {
        let offset = 36 + i * 32;
        if offset + 32 > data.len() {
            break;
        }
        if &data[offset..offset + 32] == dest_address.as_ref() {
            found = true;
            break;
        }
    }

    if !found {
        log("Destination not in whitelist!");
        return Err(ProgramError::InvalidArgument);
    }

    log("Transfer allowed: destination is whitelisted");
    Ok(())
}

// ---------------------------------------------------------------------------
// AddToWhitelist
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    pub signer: &'info Signer,
    pub new_account: &'info UncheckedAccount,
    #[account(mut)]
    pub white_list: &'info mut UncheckedAccount,
}

#[inline(always)]
pub fn handle_add_to_whitelist(accounts: &AddToWhitelist) -> Result<(), ProgramError> {
    let view = unsafe {
        &mut *(accounts.white_list as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = view.try_borrow_mut()?;

    if data.len() < 36 {
        return Err(ProgramError::AccountDataTooSmall);
    }

    // Verify signer is the authority
    let signer_address = accounts.signer.to_account_view().address();
    if &data[0..32] != signer_address.as_ref() {
        log("Only the authority can add to the whitelist");
        return Err(ProgramError::IllegalOwner);
    }

    // Read current count
    let mut count_bytes = [0u8; 4];
    count_bytes.copy_from_slice(&data[32..36]);
    let count = u32::from_le_bytes(count_bytes) as usize;

    // Write new address
    let offset = 36 + count * 32;
    if offset + 32 > data.len() {
        log("Whitelist is full");
        return Err(ProgramError::AccountDataTooSmall);
    }

    let new_address = accounts.new_account.to_account_view().address();
    data[offset..offset + 32].copy_from_slice(new_address.as_ref());

    // Update count
    let new_count = (count + 1) as u32;
    data[32..36].copy_from_slice(&new_count.to_le_bytes());

    log("Address added to whitelist");
    Ok(())
}
