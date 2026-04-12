#![cfg_attr(not(test), no_std)]

use quasar_lang::{
    cpi::{CpiCall, InstructionAccount},
    prelude::*,
};

#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

/// Correct Token-2022 program ID.
///
/// quasar-spl 0.0.0 ships incorrect bytes for the Token-2022 address
/// (`TokenzSRvw8aVrEuYKv3gLJaYV39h1EWGpCCGYBJPZQ` instead of the real
/// `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`). We define a local
/// marker with the correct mainnet address until that's fixed upstream.
pub struct Token2022Program;

impl Id for Token2022Program {
    const ID: Address = Address::new_from_array([
        6, 221, 246, 225, 238, 117, 143, 222, 24, 66, 93, 188, 228, 108, 205, 218,
        182, 26, 252, 77, 131, 185, 13, 39, 254, 189, 249, 40, 216, 161, 139, 252,
    ]);
}

/// Demonstrates Token-2022 basics: minting tokens and transferring (checked)
/// via raw CPI to the Token-2022 program.
#[program]
mod quasar_token_2022_basics {
    use super::*;

    /// Mint tokens to a recipient's token account.
    #[instruction(discriminator = 0)]
    pub fn mint_token(ctx: Ctx<MintToken>, amount: u64) -> Result<(), ProgramError> {
        handle_mint_token(&mut ctx.accounts, amount)
    }

    /// Transfer tokens using transfer_checked (required for Token-2022).
    #[instruction(discriminator = 1)]
    pub fn transfer_token(ctx: Ctx<TransferToken>, amount: u64) -> Result<(), ProgramError> {
        handle_transfer_token(&mut ctx.accounts, amount)
    }
}

/// Accounts for minting tokens via Token-2022.
#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub authority: &'info Signer,
    #[account(mut)]
    pub mint: &'info mut UncheckedAccount,
    #[account(mut)]
    pub receiver: &'info mut UncheckedAccount,
    pub token_program: &'info Program<Token2022Program>,
}

#[inline(always)]
pub fn handle_mint_token(accounts: &mut MintToken, amount: u64) -> Result<(), ProgramError> {
    // SPL Token MintTo instruction: opcode 7, amount as u64 LE.
    let data = build_u64_data(7, amount);
    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [
            InstructionAccount::writable(accounts.mint.to_account_view().address()),
            InstructionAccount::writable(accounts.receiver.to_account_view().address()),
            InstructionAccount::readonly_signer(accounts.authority.to_account_view().address()),
        ],
        [
            accounts.mint.to_account_view(),
            accounts.receiver.to_account_view(),
            accounts.authority.to_account_view(),
        ],
        data,
    )
    .invoke()
}

/// Accounts for transferring tokens via Token-2022 transfer_checked.
#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub sender: &'info Signer,
    #[account(mut)]
    pub from: &'info mut UncheckedAccount,
    pub mint: &'info UncheckedAccount,
    #[account(mut)]
    pub to: &'info mut UncheckedAccount,
    pub token_program: &'info Program<Token2022Program>,
}

#[inline(always)]
pub fn handle_transfer_token(accounts: &mut TransferToken, amount: u64) -> Result<(), ProgramError> {
    // SPL Token TransferChecked instruction: opcode 12, amount as u64 LE, decimals as u8.
    let data = build_transfer_checked_data(amount, 6);
    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [
            InstructionAccount::writable(accounts.from.to_account_view().address()),
            InstructionAccount::readonly(accounts.mint.to_account_view().address()),
            InstructionAccount::writable(accounts.to.to_account_view().address()),
            InstructionAccount::readonly_signer(accounts.sender.to_account_view().address()),
        ],
        [
            accounts.from.to_account_view(),
            accounts.mint.to_account_view(),
            accounts.to.to_account_view(),
            accounts.sender.to_account_view(),
        ],
        data,
    )
    .invoke()
}

/// Build a 9-byte instruction data: [opcode, u64 LE amount].
#[inline(always)]
fn build_u64_data(opcode: u8, amount: u64) -> [u8; 9] {
    let mut data = [0u8; 9];
    data[0] = opcode;
    data[1..9].copy_from_slice(&amount.to_le_bytes());
    data
}

/// Build TransferChecked data: [12, u64 LE amount, u8 decimals].
#[inline(always)]
fn build_transfer_checked_data(amount: u64, decimals: u8) -> [u8; 10] {
    let mut data = [0u8; 10];
    data[0] = 12;
    data[1..9].copy_from_slice(&amount.to_le_bytes());
    data[9] = decimals;
    data
}
