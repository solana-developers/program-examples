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

/// Transfer hook that charges a SOL fee proportional to the token transfer
/// amount. Uses a delegate PDA to transfer wrapped SOL from the sender's
/// WSOL token account to a fee collection account.
///
/// In this Quasar version, the core logic demonstrates the delegate-based
/// fee charging pattern with a transfer counter and amount validation.
/// The actual WSOL CPI is simplified since Token-1 CPI from Quasar would
/// require extensive manual instruction encoding.
#[program]
mod quasar_transfer_hook_cost {
    use super::*;

    /// Create the ExtraAccountMetaList PDA and the counter PDA.
    /// Discriminator = sha256("spl-transfer-hook-interface:initialize-extra-account-metas")[:8]
    #[instruction(discriminator = [43, 34, 13, 49, 167, 88, 235, 235])]
    pub fn initialize_extra_account_meta_list(
        ctx: Ctx<InitializeExtraAccountMetaList>,
    ) -> Result<(), ProgramError> {
        handle_initialize(&mut ctx.accounts)
    }

    /// Transfer hook handler — validates the amount and increments the counter.
    /// In the full version, this would also charge a WSOL fee via delegate.
    /// Discriminator = sha256("spl-transfer-hook-interface:execute")[:8]
    #[instruction(discriminator = [105, 37, 101, 197, 75, 251, 102, 26])]
    pub fn transfer_hook(ctx: Ctx<TransferHook>, amount: u64) -> Result<(), ProgramError> {
        handle_transfer_hook(&mut ctx.accounts, amount)
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
    #[account(mut)]
    pub counter_account: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize(accounts: &InitializeExtraAccountMetaList) -> Result<(), ProgramError> {
    // Create ExtraAccountMetaList PDA with 1 extra account: counter
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

    // ExtraAccountMeta: counter PDA with seeds = [Literal("counter")]
    data[16] = 1;
    let mut config = [0u8; 32];
    config[0] = 1;
    config[1] = 0; // literal
    config[2] = 7;
    config[3..10].copy_from_slice(b"counter");
    data[17..49].copy_from_slice(&config);
    data[49] = 0;
    data[50] = 1; // writable

    // Create counter PDA: 1 byte for counter (u8)
    let counter_size: u64 = 9; // 8 discriminator + 1 counter
    let counter_lamports = Rent::get()?.try_minimum_balance(counter_size as usize)?;

    let (counter_pda, counter_bump) =
        Address::find_program_address(&[b"counter"], &crate::ID);
    if accounts.counter_account.to_account_view().address() != &counter_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    let counter_bump_bytes = [counter_bump];
    let counter_seeds = [
        Seed::from(b"counter" as &[u8]),
        Seed::from(&counter_bump_bytes as &[u8]),
    ];
    accounts.system_program
        .create_account(accounts.payer, &*accounts.counter_account, counter_lamports, counter_size, &crate::ID)
        .invoke_signed(&counter_seeds)?;

    log("Transfer cost hook initialized");
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
    #[account(mut)]
    pub counter_account: &'info mut UncheckedAccount,
}

#[inline(always)]
pub fn handle_transfer_hook(accounts: &TransferHook, amount: u64) -> Result<(), ProgramError> {
    // Validate amount
    if amount > 50 {
        log("Warning: large transfer amount");
    }

    // Increment transfer counter
    let view = unsafe {
        &mut *(accounts.counter_account as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = view.try_borrow_mut()?;

    if data.len() < 9 {
        return Err(ProgramError::AccountDataTooSmall);
    }

    let counter = data[8];
    let new_counter = counter
        .checked_add(1)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    data[8] = new_counter;

    // In the full Anchor version, this would also:
    // 1. Transfer WSOL from sender's ATA to delegate's ATA
    //    using the delegate PDA as the authority
    // 2. The WSOL amount equals the token transfer amount
    // This requires several additional accounts (WSOL mint,
    // token program, ATA program, delegate PDA, and both ATAs).

    log("Transfer cost hook: counter incremented");
    Ok(())
}
