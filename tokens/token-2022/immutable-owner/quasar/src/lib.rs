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

/// Creates a token account with the ImmutableOwner extension, which prevents
/// the owner of the token account from being reassigned.
#[program]
mod quasar_immutable_owner {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>) -> Result<(), ProgramError> {
        handle_initialize(&mut ctx.accounts)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    #[account(mut)]
    pub token_account: &'info Signer,
    pub mint_account: &'info UncheckedAccount,
    pub token_program: &'info Program<Token2022Program>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize(accounts: &Initialize) -> Result<(), ProgramError> {
    // 165 (base) + 1 (account type) + 4 (TLV header, ImmutableOwner is zero-size) = 170 bytes
    let account_size: u64 = 170;
    let lamports = Rent::get()?.try_minimum_balance(account_size as usize)?;

    // 1. Create account
    accounts.system_program
        .create_account(
            accounts.payer,
            accounts.token_account,
            lamports,
            account_size,
            accounts.token_program.to_account_view().address(),
        )
        .invoke()?;

    // 2. Initialize ImmutableOwner extension: opcode 22 (no additional data)
    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [InstructionAccount::writable(
            accounts.token_account.to_account_view().address(),
        )],
        [accounts.token_account.to_account_view()],
        [22u8],
    )
    .invoke()?;

    // 3. InitializeAccount3: opcode 18, owner pubkey
    let mut data = [0u8; 33];
    data[0] = 18;
    data[1..33].copy_from_slice(accounts.payer.to_account_view().address().as_ref());

    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [
            InstructionAccount::writable(accounts.token_account.to_account_view().address()),
            InstructionAccount::readonly(accounts.mint_account.to_account_view().address()),
        ],
        [
            accounts.token_account.to_account_view(),
            accounts.mint_account.to_account_view(),
        ],
        data,
    )
    .invoke()
}
