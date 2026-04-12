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

/// Creates a token account with the MemoTransfer extension (requires memos
/// on incoming transfers) and allows disabling it.
#[program]
mod quasar_memo_transfer {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>) -> Result<(), ProgramError> {
        handle_initialize(&mut ctx.accounts)
    }

    #[instruction(discriminator = 1)]
    pub fn disable(ctx: Ctx<Disable>) -> Result<(), ProgramError> {
        handle_disable(&mut ctx.accounts)
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
    // Token account + MemoTransfer extension = 300 bytes
    let account_size: u64 = 300;
    let lamports = Rent::get()?.try_minimum_balance(account_size as usize)?;

    accounts.system_program
        .create_account(
            accounts.payer,
            accounts.token_account,
            lamports,
            account_size,
            accounts.token_program.to_account_view().address(),
        )
        .invoke()?;

    // InitializeAccount3: opcode 18, owner pubkey
    let mut init_data = [0u8; 33];
    init_data[0] = 18;
    init_data[1..33].copy_from_slice(accounts.payer.to_account_view().address().as_ref());

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
        init_data,
    )
    .invoke()?;

    // MemoTransfer enable: opcode 30, sub-opcode 0
    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [
            InstructionAccount::writable(accounts.token_account.to_account_view().address()),
            InstructionAccount::readonly_signer(accounts.payer.to_account_view().address()),
        ],
        [
            accounts.token_account.to_account_view(),
            accounts.payer.to_account_view(),
        ],
        [30u8, 0],
    )
    .invoke()
}

#[derive(Accounts)]
pub struct Disable<'info> {
    #[account(mut)]
    pub owner: &'info Signer,
    #[account(mut)]
    pub token_account: &'info mut UncheckedAccount,
    pub token_program: &'info Program<Token2022Program>,
}

#[inline(always)]
pub fn handle_disable(accounts: &Disable) -> Result<(), ProgramError> {
    // MemoTransfer disable: opcode 30, sub-opcode 1
    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [
            InstructionAccount::writable(accounts.token_account.to_account_view().address()),
            InstructionAccount::readonly_signer(accounts.owner.to_account_view().address()),
        ],
        [
            accounts.token_account.to_account_view(),
            accounts.owner.to_account_view(),
        ],
        [30u8, 1],
    )
    .invoke()
}
