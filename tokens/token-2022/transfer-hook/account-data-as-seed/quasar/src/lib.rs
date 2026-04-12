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

/// Transfer hook that uses account data as a PDA seed. The counter PDA is
/// seeded by ["counter", owner_pubkey] where the owner pubkey is read from
/// the source token account data at runtime by the Token-2022 program.
#[program]
mod quasar_transfer_hook_account_data_as_seed {
    use super::*;

    /// Create the ExtraAccountMetaList PDA (with 1 extra account: counter PDA
    /// whose seed includes account data from the source token account) and the
    /// counter PDA itself.
    /// Discriminator = sha256("spl-transfer-hook-interface:initialize-extra-account-metas")[:8]
    #[instruction(discriminator = [43, 34, 13, 49, 167, 88, 235, 235])]
    pub fn initialize_extra_account_meta_list(
        ctx: Ctx<InitializeExtraAccountMetaList>,
    ) -> Result<(), ProgramError> {
        handle_initialize_extra_account_meta_list(&mut ctx.accounts)
    }

    /// Transfer hook handler — increments a per-owner counter on each transfer.
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
    /// Counter PDA: ["counter", payer_key]
    #[account(mut)]
    pub counter_account: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize_extra_account_meta_list(accounts: &InitializeExtraAccountMetaList) -> Result<(), ProgramError> {
    // ExtraAccountMetaList with 1 extra account.
    // ExtraAccountMeta for a PDA with seeds [Literal("counter"), AccountData(0, 32, 32)]:
    //   The AccountData seed resolves the owner pubkey from account_index=0
    //   (source_token) at data_index=32 (owner field offset), length=32.
    //
    // TLV layout:
    //   [8 bytes: Execute discriminator]
    //   [4 bytes: data length]
    //   [4 bytes: PodSlice count = 1]
    //   [35 bytes: ExtraAccountMeta entry]
    // Total = 51 bytes
    let meta_list_size: u64 = 51;
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
        .create_account(
            accounts.payer,
            &*accounts.extra_account_meta_list,
            lamports,
            meta_list_size,
            &crate::ID,
        )
        .invoke_signed(&seeds)?;

    // Write TLV data
    let view = unsafe {
        &mut *(accounts.extra_account_meta_list as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = view.try_borrow_mut()?;

    data[0..8].copy_from_slice(&EXECUTE_DISCRIMINATOR);
    data[8..12].copy_from_slice(&39u32.to_le_bytes()); // data length: 4 + 35
    data[12..16].copy_from_slice(&1u32.to_le_bytes()); // count = 1

    // ExtraAccountMeta for counter PDA seeded by ["counter", AccountData(0, 32, 32)]
    data[16] = 1; // discriminator: PDA from seeds
    let mut config = [0u8; 32];
    config[0] = 2; // number of seeds
    // Seed 0: Literal "counter"
    config[1] = 0; // seed type: literal
    config[2] = 7; // seed length
    config[3..10].copy_from_slice(b"counter");
    // Seed 1: AccountData(account_index=0, data_index=32, length=32)
    config[10] = 1; // seed type: account data
    config[11] = 0; // account_index
    config[12] = 32; // data_index
    config[13] = 32; // length
    data[17..49].copy_from_slice(&config);
    data[49] = 0; // is_signer = false
    data[50] = 1; // is_writable = true

    // Create the counter PDA (seeded by payer key for this init)
    let payer_address = accounts.payer.to_account_view().address();
    let counter_size: u64 = 16;
    let counter_lamports = Rent::get()?.try_minimum_balance(counter_size as usize)?;

    let (counter_pda, counter_bump) =
        Address::find_program_address(&[b"counter", payer_address.as_ref()], &crate::ID);

    if accounts.counter_account.to_account_view().address() != &counter_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    let counter_bump_bytes = [counter_bump];
    let counter_seeds = [
        Seed::from(b"counter" as &[u8]),
        Seed::from(payer_address.as_ref()),
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
// TransferHook
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct TransferHook<'info> {
    pub source_token: &'info UncheckedAccount,
    pub mint: &'info UncheckedAccount,
    pub destination_token: &'info UncheckedAccount,
    pub owner: &'info UncheckedAccount,
    pub extra_account_meta_list: &'info UncheckedAccount,
    /// Counter PDA resolved by Token-2022 using account data seeds
    #[account(mut)]
    pub counter_account: &'info mut UncheckedAccount,
}

#[inline(always)]
pub fn handle_transfer_hook(accounts: &TransferHook) -> Result<(), ProgramError> {
    let view = unsafe {
        &mut *(accounts.counter_account as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = view.try_borrow_mut()?;

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

    log("Transfer hook: per-owner counter incremented");
    Ok(())
}
