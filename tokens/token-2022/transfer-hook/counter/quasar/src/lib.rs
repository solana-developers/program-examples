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
/// Execute: sha256("spl-transfer-hook-interface:execute")[:8]
#[allow(dead_code)]
const EXECUTE_DISCRIMINATOR: [u8; 8] = [105, 37, 101, 197, 75, 251, 102, 26];

/// Transfer hook that counts how many times a token has been transferred.
/// The counter is stored in a PDA seeded by ["counter"].
#[program]
mod quasar_transfer_hook_counter {
    use super::*;

    /// Create the ExtraAccountMetaList PDA (with 1 extra account: the counter PDA)
    /// and the counter PDA itself.
    /// Discriminator = sha256("spl-transfer-hook-interface:initialize-extra-account-metas")[:8]
    #[instruction(discriminator = [43, 34, 13, 49, 167, 88, 235, 235])]
    pub fn initialize_extra_account_meta_list(
        ctx: Ctx<InitializeExtraAccountMetaList>,
    ) -> Result<(), ProgramError> {
        handle_initialize_extra_account_meta_list(&mut ctx.accounts)
    }

    /// Transfer hook handler — increments the counter on each transfer.
    /// Discriminator = sha256("spl-transfer-hook-interface:execute")[:8]
    #[instruction(discriminator = [105, 37, 101, 197, 75, 251, 102, 26])]
    pub fn transfer_hook(ctx: Ctx<TransferHook>, _amount: u64) -> Result<(), ProgramError> {
        handle_transfer_hook(&mut ctx.accounts)
    }
}

// ---------------------------------------------------------------------------
// InitializeExtraAccountMetaList
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    /// ExtraAccountMetaList PDA: ["extra-account-metas", mint]
    #[account(mut)]
    pub extra_account_meta_list: &'info mut UncheckedAccount,
    pub mint: &'info UncheckedAccount,
    /// Counter PDA: ["counter"]
    #[account(mut)]
    pub counter_account: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize_extra_account_meta_list(accounts: &InitializeExtraAccountMetaList) -> Result<(), ProgramError> {
    // ExtraAccountMetaList with 1 extra account:
    //   [8 bytes: Execute discriminator]
    //   [4 bytes: data length]
    //   [4 bytes: PodSlice count = 1]
    //   [35 bytes: ExtraAccountMeta entry for the counter PDA]
    // Total = 8 + 4 + 4 + 35 = 51 bytes
    let meta_list_size: u64 = 51;
    let lamports = Rent::get()?.try_minimum_balance(meta_list_size as usize)?;

    // Derive ExtraAccountMetaList PDA
    let mint_address = accounts.mint.to_account_view().address();
    let (expected_pda, bump) = Address::find_program_address(
        &[b"extra-account-metas", mint_address.as_ref()],
        &crate::ID,
    );

    let meta_list_address = accounts.extra_account_meta_list.to_account_view().address();
    if meta_list_address != &expected_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    // Create ExtraAccountMetaList PDA
    let bump_bytes = [bump];
    let seeds = [
        Seed::from(b"extra-account-metas" as &[u8]),
        Seed::from(mint_address.as_ref()),
        Seed::from(&bump_bytes as &[u8]),
    ];

    accounts.system_program
        .create_account(
            accounts.payer,
            &*accounts.extra_account_meta_list,
            lamports,
            meta_list_size,
            &crate::ID,
        )
        .invoke_signed(&seeds)?;

    // Write TLV data with the counter PDA as an extra account
    let view = unsafe {
        &mut *(accounts.extra_account_meta_list as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = view.try_borrow_mut()?;

    // Execute discriminator (TLV type tag)
    data[0..8].copy_from_slice(&EXECUTE_DISCRIMINATOR);
    // Data length: 4 (count) + 35 (one ExtraAccountMeta) = 39
    data[8..12].copy_from_slice(&39u32.to_le_bytes());
    // PodSlice count: 1 entry
    data[12..16].copy_from_slice(&1u32.to_le_bytes());

    // ExtraAccountMeta for counter PDA (35 bytes):
    // [0]: discriminator (1 = PDA from seeds)
    // [1]: address_config (32 bytes encoding the seeds)
    // [33]: is_signer (0)
    // [34]: is_writable (1)
    //
    // For a PDA with seeds = [Literal("counter")], the address_config
    // uses the ExtraAccountMeta seed encoding format. The seeds are:
    //   Seed::Literal { bytes: b"counter" }
    // Encoded as: [length: 1 byte][data: N bytes]
    //
    // The full ExtraAccountMeta seed-based encoding:
    //   discriminator = 1 (PDA)
    //   address_config[0] = 1 (number of seeds)
    //   address_config[1] = 0 (seed type: literal)
    //   address_config[2] = 7 (seed length)
    //   address_config[3..10] = b"counter"
    //   address_config[10..32] = zeroes (padding)
    //   is_signer = 0
    //   is_writable = 1
    data[16] = 1; // discriminator: PDA from seeds
    let mut config = [0u8; 32];
    config[0] = 1; // number of seeds
    config[1] = 0; // seed type: literal
    config[2] = 7; // seed length
    config[3..10].copy_from_slice(b"counter");
    data[17..49].copy_from_slice(&config);
    data[49] = 0; // is_signer = false
    data[50] = 1; // is_writable = true

    // Also create the counter PDA (8 bytes for u64 counter + 8 bytes discriminator)
    let counter_size: u64 = 16;
    let counter_lamports = Rent::get()?.try_minimum_balance(counter_size as usize)?;

    let (counter_pda, counter_bump) =
        Address::find_program_address(&[b"counter"], &crate::ID);

    let counter_address = accounts.counter_account.to_account_view().address();
    if counter_address != &counter_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    let counter_bump_bytes = [counter_bump];
    let counter_seeds = [
        Seed::from(b"counter" as &[u8]),
        Seed::from(&counter_bump_bytes as &[u8]),
    ];

    accounts.system_program
        .create_account(
            accounts.payer,
            &*accounts.counter_account,
            counter_lamports,
            counter_size,
            &crate::ID,
        )
        .invoke_signed(&counter_seeds)?;

    log("Extra account meta list and counter initialized");
    Ok(())
}

// ---------------------------------------------------------------------------
// TransferHook: increment the counter on each transfer
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct TransferHook<'info> {
    /// Source token account
    pub source_token: &'info UncheckedAccount,
    /// Mint
    pub mint: &'info UncheckedAccount,
    /// Destination token account
    pub destination_token: &'info UncheckedAccount,
    /// Source token account owner
    pub owner: &'info UncheckedAccount,
    /// ExtraAccountMetaList PDA
    pub extra_account_meta_list: &'info UncheckedAccount,
    /// Counter PDA (extra account resolved by Token-2022)
    #[account(mut)]
    pub counter_account: &'info mut UncheckedAccount,
}

#[inline(always)]
pub fn handle_transfer_hook(accounts: &TransferHook) -> Result<(), ProgramError> {
    // Read the current counter from the account data
    let view = unsafe {
        &mut *(accounts.counter_account as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = view.try_borrow_mut()?;

    // Counter is at offset 8 (after 8-byte Anchor-style discriminator)
    // In our case we just use the first 8 bytes as the counter
    if data.len() < 16 {
        return Err(ProgramError::AccountDataTooSmall);
    }

    let mut counter_bytes = [0u8; 8];
    counter_bytes.copy_from_slice(&data[8..16]);
    let counter = u64::from_le_bytes(counter_bytes);

    let new_counter = counter
        .checked_add(1)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    data[8..16].copy_from_slice(&new_counter.to_le_bytes());

    log("Transfer hook: counter incremented");
    Ok(())
}
