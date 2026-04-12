#![cfg_attr(not(test), no_std)]

use quasar_lang::sysvars::Sysvar;
use quasar_lang::{
    cpi::{CpiCall, InstructionAccount},
    prelude::*,
};

#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

pub struct Token2022Program;
impl Id for Token2022Program {
    const ID: Address = Address::new_from_array([
        6, 221, 246, 225, 238, 117, 143, 222, 24, 66, 93, 188, 228, 108, 205, 218,
        182, 26, 252, 77, 131, 185, 13, 39, 254, 189, 249, 40, 216, 161, 139, 252,
    ]);
}

/// SPL Transfer Hook Interface discriminators (SHA-256 prefix).
/// Execute: sha256("spl-transfer-hook-interface:execute")[:8]
const EXECUTE_DISCRIMINATOR: [u8; 8] = [105, 37, 101, 197, 75, 251, 102, 26];
/// InitializeExtraAccountMetaList:
/// sha256("spl-transfer-hook-interface:initialize-extra-account-metas")[:8]
#[allow(dead_code)]
const INIT_EXTRA_ACCOUNT_METAS_DISCRIMINATOR: [u8; 8] = [43, 34, 13, 49, 167, 88, 235, 235];

/// Demonstrates the TransferHook extension: a minimal hook that logs a
/// message during every token transfer. No extra accounts are required.
#[program]
mod quasar_transfer_hook_hello_world {
    use super::*;

    /// Create a mint with the TransferHook extension pointing to this program.
    /// Custom discriminator (not part of the transfer hook interface).
    #[instruction(discriminator = [0, 0, 0, 0, 0, 0, 0, 1])]
    pub fn initialize(ctx: Ctx<Initialize>, decimals: u8) -> Result<(), ProgramError> {
        handle_initialize(&mut ctx.accounts, decimals)
    }

    /// Create the ExtraAccountMetaList PDA (empty — no extra accounts).
    /// Discriminator = sha256("spl-transfer-hook-interface:initialize-extra-account-metas")[:8]
    #[instruction(discriminator = [43, 34, 13, 49, 167, 88, 235, 235])]
    pub fn initialize_extra_account_meta_list(
        ctx: Ctx<InitializeExtraAccountMetaList>,
    ) -> Result<(), ProgramError> {
        handle_initialize_extra_account_meta_list(&mut ctx.accounts)
    }

    /// Transfer hook handler — called automatically by Token-2022 during transfers.
    /// Discriminator = sha256("spl-transfer-hook-interface:execute")[:8]
    #[instruction(discriminator = [105, 37, 101, 197, 75, 251, 102, 26])]
    pub fn transfer_hook(ctx: Ctx<TransferHook>, _amount: u64) -> Result<(), ProgramError> {
        handle_transfer_hook(&mut ctx.accounts)
    }
}

// ---------------------------------------------------------------------------
// Initialize: create a mint with TransferHook extension
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    #[account(mut)]
    pub mint_account: &'info Signer,
    pub token_program: &'info Program<Token2022Program>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize(accounts: &Initialize, decimals: u8) -> Result<(), ProgramError> {
    // Mint with TransferHook extension:
    //   165 (base account + padding) + 1 (account type) + 4 (TLV header) + 64 (extension) = 234
    let mint_size: u64 = 234;
    let lamports = Rent::get()?.try_minimum_balance(mint_size as usize)?;

    // 1. Create account owned by Token-2022
    accounts.system_program
        .create_account(
            accounts.payer,
            accounts.mint_account,
            lamports,
            mint_size,
            accounts.token_program.to_account_view().address(),
        )
        .invoke()?;

    // 2. InitializeTransferHook extension
    // Layout: [36u8 (TransferHookExtension), 0u8 (Initialize),
    //          authority(32), program_id(32)]
    let mut ext_data = [0u8; 66];
    ext_data[0] = 36; // TokenInstruction::TransferHookExtension
    ext_data[1] = 0; // TransferHookInstruction::Initialize
    ext_data[2..34].copy_from_slice(accounts.payer.to_account_view().address().as_ref());
    ext_data[34..66].copy_from_slice(crate::ID.as_ref());

    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [InstructionAccount::writable(
            accounts.mint_account.to_account_view().address(),
        )],
        [accounts.mint_account.to_account_view()],
        ext_data,
    )
    .invoke()?;

    // 3. InitializeMint2: opcode 20
    let mut mint_data = [0u8; 67];
    mint_data[0] = 20;
    mint_data[1] = decimals;
    mint_data[2..34].copy_from_slice(accounts.payer.to_account_view().address().as_ref());
    mint_data[34] = 1; // has freeze authority
    mint_data[35..67].copy_from_slice(accounts.payer.to_account_view().address().as_ref());

    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [InstructionAccount::writable(
            accounts.mint_account.to_account_view().address(),
        )],
        [accounts.mint_account.to_account_view()],
        mint_data,
    )
    .invoke()
}

// ---------------------------------------------------------------------------
// InitializeExtraAccountMetaList: create the TLV account for extra accounts
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    /// ExtraAccountMetaList PDA seeded by ["extra-account-metas", mint]
    #[account(mut)]
    pub extra_account_meta_list: &'info mut UncheckedAccount,
    pub mint: &'info UncheckedAccount,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize_extra_account_meta_list(accounts: &InitializeExtraAccountMetaList) -> Result<(), ProgramError> {
    use quasar_lang::cpi::Seed;

    // ExtraAccountMetaList with 0 extra accounts:
    //   [8 bytes: Execute discriminator]
    //   [4 bytes: data length = 4]
    //   [4 bytes: PodSlice count = 0]
    // Total = 16 bytes
    let meta_list_size: u64 = 16;
    let lamports = Rent::get()?.try_minimum_balance(meta_list_size as usize)?;

    // Derive PDA
    let mint_address = accounts.mint.to_account_view().address();
    let (expected_pda, bump) = Address::find_program_address(
        &[b"extra-account-metas", mint_address.as_ref()],
        &crate::ID,
    );

    let meta_list_address = accounts.extra_account_meta_list.to_account_view().address();
    if meta_list_address != &expected_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    // Create PDA account owned by this program
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

    // Write TLV data into the account.
    // SAFETY: Account was just created (16 bytes) and is owned by this program.
    // UncheckedAccount is #[repr(transparent)] over AccountView, so the cast is safe.
    let view = unsafe {
        &mut *(accounts.extra_account_meta_list as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = view.try_borrow_mut()?;
    // Execute discriminator (type tag in TLV)
    data[0..8].copy_from_slice(&EXECUTE_DISCRIMINATOR);
    // Data length: 4 bytes for the PodSlice count field
    data[8..12].copy_from_slice(&4u32.to_le_bytes());
    // PodSlice count: 0 entries
    data[12..16].copy_from_slice(&0u32.to_le_bytes());

    log("Extra account meta list initialized");
    Ok(())
}

// ---------------------------------------------------------------------------
// TransferHook: the hook handler invoked during token transfers
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
}

#[inline(always)]
pub fn handle_transfer_hook(accounts: &TransferHook) -> Result<(), ProgramError> {
    // In production, verify the source token's TransferHookAccount.transferring
    // flag is set. The Token-2022 program sets this before invoking the hook
    // and clears it after, preventing standalone invocation.
    //
    // For this hello-world example, we simply log a message.
    log("Hello Transfer Hook!");
    Ok(())
}
