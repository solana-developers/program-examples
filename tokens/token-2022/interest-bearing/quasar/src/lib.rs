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

/// Creates a mint with the InterestBearingConfig extension and allows
/// updating the interest rate.
#[program]
mod quasar_interest_bearing {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>, rate: i16) -> Result<(), ProgramError> {
        handle_initialize(&mut ctx.accounts, rate)
    }

    #[instruction(discriminator = 1)]
    pub fn update_rate(ctx: Ctx<UpdateRate>, rate: i16) -> Result<(), ProgramError> {
        handle_update_rate(&mut ctx.accounts, rate)
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
pub fn handle_initialize(accounts: &Initialize, rate: i16) -> Result<(), ProgramError> {
    // 165 (base) + 1 (account type) + 4 (TLV header) + 52 (InterestBearingConfig data) = 222 bytes
    let mint_size: u64 = 222;
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

    // InterestBearingMintInitialize: opcode 33, sub-opcode 0
    // Data: [33, 0, rate_authority (32 bytes), rate (i16 LE)]
    let mut ext_data = [0u8; 36];
    ext_data[0] = 33;
    ext_data[1] = 0; // Initialize sub-opcode
    ext_data[2..34].copy_from_slice(accounts.payer.to_account_view().address().as_ref());
    ext_data[34..36].copy_from_slice(&rate.to_le_bytes());

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
pub struct UpdateRate<'info> {
    #[account(mut)]
    pub authority: &'info Signer,
    #[account(mut)]
    pub mint_account: &'info mut UncheckedAccount,
    pub token_program: &'info Program<Token2022Program>,
}

#[inline(always)]
pub fn handle_update_rate(accounts: &UpdateRate, rate: i16) -> Result<(), ProgramError> {
    // InterestBearingMintUpdateRate: opcode 33, sub-opcode 1, rate (i16 LE)
    let mut data = [0u8; 4];
    data[0] = 33;
    data[1] = 1;
    data[2..4].copy_from_slice(&rate.to_le_bytes());

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
