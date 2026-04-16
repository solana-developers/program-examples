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

/// Demonstrates the MintCloseAuthority extension which allows closing mint
/// accounts (normally impossible). Includes close functionality via CPI.
#[program]
mod quasar_mint_close_authority {
    use super::*;

    /// Create a mint with the MintCloseAuthority extension.
    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>) -> Result<(), ProgramError> {
        handle_initialize(&mut ctx.accounts)
    }

    /// Close the mint account, reclaiming lamports to the authority.
    #[instruction(discriminator = 1)]
    pub fn close(ctx: Ctx<Close>) -> Result<(), ProgramError> {
        handle_close(&mut ctx.accounts)
    }
}

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
pub fn handle_initialize(accounts: &Initialize) -> Result<(), ProgramError> {
    // 165 (base) + 1 (account type) + 4 (TLV header) + 32 (MintCloseAuthority data) = 202 bytes
    let mint_size: u64 = 202;
    let lamports = Rent::get()?.try_minimum_balance(mint_size as usize)?;

    accounts.system_program
        .create_account(
            accounts.payer,
            accounts.mint_account,
            lamports,
            mint_size,
            accounts.token_program.to_account_view().address(),
        )
        .invoke()?;

    // InitializeMintCloseAuthority: opcode 25, COption::Some flag (1 byte), close_authority pubkey (32 bytes)
    let mut ext_data = [0u8; 34];
    ext_data[0] = 25; // InitializeMintCloseAuthority
    ext_data[1] = 1;  // COption::Some
    ext_data[2..34].copy_from_slice(accounts.payer.to_account_view().address().as_ref());

    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [InstructionAccount::writable(
            accounts.mint_account.to_account_view().address(),
        )],
        [accounts.mint_account.to_account_view()],
        ext_data,
    )
    .invoke()?;

    // InitializeMint2
    let mut mint_data = [0u8; 67];
    mint_data[0] = 20;
    mint_data[1] = 2;
    mint_data[2..34].copy_from_slice(accounts.payer.to_account_view().address().as_ref());
    mint_data[34] = 0; // no freeze authority

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

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub authority: &'info Signer,
    #[account(mut)]
    pub mint_account: &'info mut UncheckedAccount,
    pub token_program: &'info Program<Token2022Program>,
}

#[inline(always)]
pub fn handle_close(accounts: &Close) -> Result<(), ProgramError> {
    // CloseAccount: opcode 9
    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [
            InstructionAccount::writable(accounts.mint_account.to_account_view().address()),
            InstructionAccount::writable(accounts.authority.to_account_view().address()),
            InstructionAccount::readonly_signer(accounts.authority.to_account_view().address()),
        ],
        [
            accounts.mint_account.to_account_view(),
            accounts.authority.to_account_view(),
            accounts.authority.to_account_view(),
        ],
        [9u8],
    )
    .invoke()
}
