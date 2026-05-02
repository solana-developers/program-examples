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

/// Transfer hook with an on/off switch per wallet. An admin can enable or
/// disable transfers for individual wallets. The hook checks the sender's
/// switch account before allowing a transfer.
#[program]
mod quasar_transfer_hook_switch {
    use super::*;

    /// Set up or change the admin. The first caller becomes admin.
    #[instruction(discriminator = [0, 0, 0, 0, 0, 0, 0, 1])]
    pub fn configure_admin(ctx: Ctx<ConfigureAdmin>) -> Result<(), ProgramError> {
        handle_configure_admin(&mut ctx.accounts)
    }

    /// Create the ExtraAccountMetaList PDA.
    /// Discriminator = sha256("spl-transfer-hook-interface:initialize-extra-account-metas")[:8]
    #[instruction(discriminator = [43, 34, 13, 49, 167, 88, 235, 235])]
    pub fn initialize_extra_account_metas_list(
        ctx: Ctx<InitializeExtraAccountMetas>,
    ) -> Result<(), ProgramError> {
        handle_initialize(&mut ctx.accounts)
    }

    /// Toggle the transfer switch for a wallet.
    #[instruction(discriminator = [0, 0, 0, 0, 0, 0, 0, 3])]
    pub fn switch(ctx: Ctx<Switch>, on: u8) -> Result<(), ProgramError> {
        handle_switch(&mut ctx.accounts, on != 0)
    }

    /// Transfer hook handler — checks the sender's switch is on.
    /// Discriminator = sha256("spl-transfer-hook-interface:execute")[:8]
    #[instruction(discriminator = [105, 37, 101, 197, 75, 251, 102, 26])]
    pub fn transfer_hook(ctx: Ctx<TransferHook>, _amount: u64) -> Result<(), ProgramError> {
        handle_transfer_hook(&mut ctx.accounts)
    }
}

// ---------------------------------------------------------------------------
// AdminConfig: [32 bytes admin pubkey] [1 byte is_initialised]
// WalletSwitch: [32 bytes wallet pubkey] [1 byte on]
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ConfigureAdmin
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct ConfigureAdmin<'info> {
    #[account(mut)]
    pub admin: &'info Signer,
    pub new_admin: &'info UncheckedAccount,
    #[account(mut)]
    pub admin_config: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_configure_admin(accounts: &ConfigureAdmin) -> Result<(), ProgramError> {
    let view = accounts.admin_config.to_account_view();
    let data = view.try_borrow()?;

    // If already initialised, verify caller is the current admin
    if data.len() >= 33 && data[32] != 0 {
        let admin_address = accounts.admin.to_account_view().address();
        if &data[0..32] != admin_address.as_ref() {
            log("Only the current admin can change the admin");
            return Err(ProgramError::IllegalOwner);
        }
    }
    drop(data);

    // Create or reuse admin_config PDA
    let (admin_config_pda, bump) =
        Address::find_program_address(&[b"admin-config"], &crate::ID);
    if accounts.admin_config.to_account_view().address() != &admin_config_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    // If account doesn't exist, create it
    if accounts.admin_config.to_account_view().data_len() == 0 {
        let size: u64 = 33; // 32 admin + 1 flag
        let lamports = Rent::get()?.try_minimum_balance(size as usize)?;
        let bump_bytes = [bump];
        let seeds = [
            Seed::from(b"admin-config" as &[u8]),
            Seed::from(&bump_bytes as &[u8]),
        ];
        accounts.system_program
            .create_account(accounts.admin, &*accounts.admin_config, lamports, size, &crate::ID)
            .invoke_signed(&seeds)?;
    }

    // Write new admin
    let mview = unsafe {
        &mut *(accounts.admin_config as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = mview.try_borrow_mut()?;
    let new_admin_address = accounts.new_admin.to_account_view().address();
    data[0..32].copy_from_slice(new_admin_address.as_ref());
    data[32] = 1; // is_initialised

    log("Admin configured");
    Ok(())
}

// ---------------------------------------------------------------------------
// InitializeExtraAccountMetas
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct InitializeExtraAccountMetas<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    pub token_mint: &'info UncheckedAccount,
    #[account(mut)]
    pub extra_account_metas_list: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize(accounts: &InitializeExtraAccountMetas) -> Result<(), ProgramError> {
    // 1 extra account: wallet switch PDA seeded by [AccountKey(index=3)] (sender/owner)
    let meta_list_size: u64 = 51; // 8 + 4 + 4 + 35
    let lamports = Rent::get()?.try_minimum_balance(meta_list_size as usize)?;

    let mint_address = accounts.token_mint.to_account_view().address();
    let (expected_pda, bump) = Address::find_program_address(
        &[b"extra-account-metas", mint_address.as_ref()],
        &crate::ID,
    );
    if accounts.extra_account_metas_list.to_account_view().address() != &expected_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    let bump_bytes = [bump];
    let seeds = [
        Seed::from(b"extra-account-metas" as &[u8]),
        Seed::from(mint_address.as_ref()),
        Seed::from(&bump_bytes as &[u8]),
    ];

    accounts.system_program
        .create_account(accounts.payer, &*accounts.extra_account_metas_list, lamports, meta_list_size, &crate::ID)
        .invoke_signed(&seeds)?;

    let view = unsafe {
        &mut *(accounts.extra_account_metas_list as *const UncheckedAccount
            as *mut UncheckedAccount as *mut AccountView)
    };
    let mut data = view.try_borrow_mut()?;
    data[0..8].copy_from_slice(&EXECUTE_DISCRIMINATOR);
    data[8..12].copy_from_slice(&39u32.to_le_bytes());
    data[12..16].copy_from_slice(&1u32.to_le_bytes());

    // ExtraAccountMeta: PDA seeded by [AccountKey(index=3)] — the sender/owner
    data[16] = 1; // PDA from seeds
    let mut config = [0u8; 32];
    config[0] = 1; // 1 seed
    config[1] = 2; // seed type: account key
    config[2] = 3; // account index 3 (owner/sender)
    data[17..49].copy_from_slice(&config);
    data[49] = 0; // not signer
    data[50] = 0; // not writable (just reading switch state)

    log("Extra account metas list initialized");
    Ok(())
}

// ---------------------------------------------------------------------------
// Switch
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct Switch<'info> {
    #[account(mut)]
    pub admin: &'info Signer,
    pub wallet: &'info UncheckedAccount,
    pub admin_config: &'info UncheckedAccount,
    #[account(mut)]
    pub wallet_switch: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_switch(accounts: &Switch, on: bool) -> Result<(), ProgramError> {
    // Verify admin
    let config_view = accounts.admin_config.to_account_view();
    let config_data = config_view.try_borrow()?;
    if config_data.len() < 33 || config_data[32] == 0 {
        return Err(ProgramError::UninitializedAccount);
    }
    let admin_address = accounts.admin.to_account_view().address();
    if &config_data[0..32] != admin_address.as_ref() {
        log("Only admin can switch");
        return Err(ProgramError::IllegalOwner);
    }
    drop(config_data);

    // Create wallet switch PDA if needed
    let wallet_address = accounts.wallet.to_account_view().address();
    let (switch_pda, switch_bump) =
        Address::find_program_address(&[wallet_address.as_ref()], &crate::ID);
    if accounts.wallet_switch.to_account_view().address() != &switch_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    if accounts.wallet_switch.to_account_view().data_len() == 0 {
        let size: u64 = 33; // 32 wallet + 1 on
        let lamports = Rent::get()?.try_minimum_balance(size as usize)?;
        let switch_bump_bytes = [switch_bump];
        let switch_seeds = [
            Seed::from(wallet_address.as_ref()),
            Seed::from(&switch_bump_bytes as &[u8]),
        ];
        accounts.system_program
            .create_account(accounts.admin, &*accounts.wallet_switch, lamports, size, &crate::ID)
            .invoke_signed(&switch_seeds)?;
    }

    let mview = unsafe {
        &mut *(accounts.wallet_switch as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = mview.try_borrow_mut()?;
    data[0..32].copy_from_slice(wallet_address.as_ref());
    data[32] = if on { 1 } else { 0 };

    log("Switch toggled");
    Ok(())
}

// ---------------------------------------------------------------------------
// TransferHook
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct TransferHook<'info> {
    pub source_token_account: &'info UncheckedAccount,
    pub token_mint: &'info UncheckedAccount,
    pub receiver_token_account: &'info UncheckedAccount,
    pub wallet: &'info UncheckedAccount,
    pub extra_account_metas_list: &'info UncheckedAccount,
    /// Wallet switch PDA resolved by Token-2022
    pub wallet_switch: &'info UncheckedAccount,
}

#[inline(always)]
pub fn handle_transfer_hook(accounts: &TransferHook) -> Result<(), ProgramError> {
    let switch_view = accounts.wallet_switch.to_account_view();
    let data = switch_view.try_borrow()?;

    if data.len() < 33 {
        log("Switch not initialized — transfers disabled by default");
        return Err(ProgramError::UninitializedAccount);
    }

    if data[32] != 1 {
        log("Transfer switch is OFF");
        return Err(ProgramError::InvalidArgument);
    }

    log("Transfer switch is ON — transfer allowed");
    Ok(())
}
