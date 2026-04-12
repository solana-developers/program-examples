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
        ctx.accounts.initialize()
    }

    #[instruction(discriminator = 1)]
    pub fn disable(ctx: Ctx<Disable>) -> Result<(), ProgramError> {
        ctx.accounts.disable()
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

impl Initialize<'_> {
    #[inline(always)]
    pub fn initialize(&self) -> Result<(), ProgramError> {
        // Token account + MemoTransfer extension = 300 bytes
        let account_size: u64 = 300;
        let lamports = Rent::get()?.try_minimum_balance(account_size as usize)?;

        self.system_program
            .create_account(
                self.payer,
                self.token_account,
                lamports,
                account_size,
                self.token_program.to_account_view().address(),
            )
            .invoke()?;

        // InitializeAccount3: opcode 18, owner pubkey
        let mut init_data = [0u8; 33];
        init_data[0] = 18;
        init_data[1..33].copy_from_slice(self.payer.to_account_view().address().as_ref());

        CpiCall::new(
            self.token_program.to_account_view().address(),
            [
                InstructionAccount::writable(self.token_account.to_account_view().address()),
                InstructionAccount::readonly(self.mint_account.to_account_view().address()),
            ],
            [
                self.token_account.to_account_view(),
                self.mint_account.to_account_view(),
            ],
            init_data,
        )
        .invoke()?;

        // MemoTransfer enable: opcode 30, sub-opcode 0
        CpiCall::new(
            self.token_program.to_account_view().address(),
            [
                InstructionAccount::writable(self.token_account.to_account_view().address()),
                InstructionAccount::readonly_signer(self.payer.to_account_view().address()),
            ],
            [
                self.token_account.to_account_view(),
                self.payer.to_account_view(),
            ],
            [30u8, 0],
        )
        .invoke()
    }
}

#[derive(Accounts)]
pub struct Disable<'info> {
    #[account(mut)]
    pub owner: &'info Signer,
    #[account(mut)]
    pub token_account: &'info mut UncheckedAccount,
    pub token_program: &'info Program<Token2022Program>,
}

impl Disable<'_> {
    #[inline(always)]
    pub fn disable(&self) -> Result<(), ProgramError> {
        // MemoTransfer disable: opcode 30, sub-opcode 1
        CpiCall::new(
            self.token_program.to_account_view().address(),
            [
                InstructionAccount::writable(self.token_account.to_account_view().address()),
                InstructionAccount::readonly_signer(self.owner.to_account_view().address()),
            ],
            [
                self.token_account.to_account_view(),
                self.owner.to_account_view(),
            ],
            [30u8, 1],
        )
        .invoke()
    }
}
