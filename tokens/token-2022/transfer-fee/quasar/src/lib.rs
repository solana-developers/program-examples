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

/// Demonstrates the TransferFee extension: creating a mint with transfer fees,
/// transferring with fee, updating the fee, and withdrawing withheld fees.
#[program]
mod quasar_transfer_fee {
    use super::*;

    /// Create a mint with the TransferFeeConfig extension.
    #[instruction(discriminator = 0)]
    pub fn initialize(
        ctx: Ctx<Initialize>,
        transfer_fee_basis_points: u16,
        maximum_fee: u64,
    ) -> Result<(), ProgramError> {
        handle_initialize(&mut ctx.accounts, transfer_fee_basis_points, maximum_fee)
    }

    /// Transfer tokens with fee.
    #[instruction(discriminator = 1)]
    pub fn transfer(ctx: Ctx<Transfer>, amount: u64, fee: u64) -> Result<(), ProgramError> {
        handle_transfer(&mut ctx.accounts, amount, fee)
    }

    /// Update the transfer fee (takes effect after 2 epochs).
    #[instruction(discriminator = 2)]
    pub fn update_fee(
        ctx: Ctx<UpdateFee>,
        transfer_fee_basis_points: u16,
        maximum_fee: u64,
    ) -> Result<(), ProgramError> {
        handle_update_fee(&mut ctx.accounts, transfer_fee_basis_points, maximum_fee)
    }

    /// Withdraw withheld fees from the mint account.
    #[instruction(discriminator = 3)]
    pub fn withdraw(ctx: Ctx<Withdraw>) -> Result<(), ProgramError> {
        handle_withdraw(&mut ctx.accounts)
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
pub fn handle_initialize(accounts: &Initialize, basis_points: u16, max_fee: u64) -> Result<(), ProgramError> {
    // 165 (base) + 1 (AccountType) + 4 (TLV header) + 108 (TransferFeeConfig data) = 278 bytes
    let mint_size: u64 = 278;
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

    // TransferFeeExtension opcode 26, sub-instruction 0 = InitializeTransferFeeConfig
    // Data: [26, 0, COption_flag(1), config_authority(32), COption_flag(1), withdraw_authority(32),
    //        basis_points(u16 LE), max_fee(u64 LE)]
    let mut ext_data = [0u8; 78];
    ext_data[0] = 26; // TransferFeeExtension
    ext_data[1] = 0;  // InitializeTransferFeeConfig sub-instruction
    ext_data[2] = 1;  // COption::Some for config_authority
    ext_data[3..35].copy_from_slice(accounts.payer.to_account_view().address().as_ref());
    ext_data[35] = 1; // COption::Some for withdraw_authority
    ext_data[36..68].copy_from_slice(accounts.payer.to_account_view().address().as_ref());
    ext_data[68..70].copy_from_slice(&basis_points.to_le_bytes());
    ext_data[70..78].copy_from_slice(&max_fee.to_le_bytes());

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
    mint_data[34] = 1;
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

#[derive(Accounts)]
pub struct Transfer<'info> {
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
pub fn handle_transfer(accounts: &mut Transfer, amount: u64, fee: u64) -> Result<(), ProgramError> {
    // TransferCheckedWithFee: opcode 37
    // Data: [37, amount (u64 LE), decimals (u8), fee (u64 LE)]
    let mut data = [0u8; 18];
    data[0] = 37;
    data[1..9].copy_from_slice(&amount.to_le_bytes());
    data[9] = 2; // decimals
    data[10..18].copy_from_slice(&fee.to_le_bytes());

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

#[derive(Accounts)]
pub struct UpdateFee<'info> {
    pub authority: &'info Signer,
    #[account(mut)]
    pub mint_account: &'info mut UncheckedAccount,
    pub token_program: &'info Program<Token2022Program>,
}

#[inline(always)]
pub fn handle_update_fee(accounts: &UpdateFee, basis_points: u16, max_fee: u64) -> Result<(), ProgramError> {
    // SetTransferFee: opcode 26, sub-opcode 4
    // Actually: extension instruction layout is different.
    // TransferFeeInstruction::SetTransferFee = 4 within type 26
    let mut data = [0u8; 12];
    data[0] = 26;
    data[1] = 4; // SetTransferFee sub-instruction
    data[2..4].copy_from_slice(&basis_points.to_le_bytes());
    data[4..12].copy_from_slice(&max_fee.to_le_bytes());

    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [
            InstructionAccount::writable(accounts.mint_account.to_account_view().address()),
            InstructionAccount::readonly_signer(accounts.authority.to_account_view().address()),
        ],
        [
            accounts.mint_account.to_account_view(),
            accounts.authority.to_account_view(),
        ],
        data,
    )
    .invoke()
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub authority: &'info Signer,
    #[account(mut)]
    pub mint_account: &'info mut UncheckedAccount,
    #[account(mut)]
    pub destination: &'info mut UncheckedAccount,
    pub token_program: &'info Program<Token2022Program>,
}

#[inline(always)]
pub fn handle_withdraw(accounts: &Withdraw) -> Result<(), ProgramError> {
    // WithdrawWithheldTokensFromMint: opcode 26, sub-opcode 3
    let data: [u8; 2] = [26, 3];

    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [
            InstructionAccount::writable(accounts.mint_account.to_account_view().address()),
            InstructionAccount::writable(accounts.destination.to_account_view().address()),
            InstructionAccount::readonly_signer(accounts.authority.to_account_view().address()),
        ],
        [
            accounts.mint_account.to_account_view(),
            accounts.destination.to_account_view(),
            accounts.authority.to_account_view(),
        ],
        data,
    )
    .invoke()
}
