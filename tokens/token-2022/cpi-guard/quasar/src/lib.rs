#![cfg_attr(not(test), no_std)]

use quasar_lang::{
    cpi::{CpiCall, InstructionAccount},
    prelude::*,
};

#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

/// Correct Token-2022 program ID (quasar-spl 0.0.0 has wrong bytes).
pub struct Token2022Program;
impl Id for Token2022Program {
    const ID: Address = Address::new_from_array([
        6, 221, 246, 225, 238, 117, 143, 222, 24, 66, 93, 188, 228, 108, 205, 218,
        182, 26, 252, 77, 131, 185, 13, 39, 254, 189, 249, 40, 216, 161, 139, 252,
    ]);
}

/// CPI Guard prevents delegated transfers via CPI. This program demonstrates
/// that a CPI transfer_checked call will fail when the sender's token account
/// has the CPI Guard extension enabled.
#[program]
mod quasar_cpi_guard {
    use super::*;

    /// Attempt a CPI transfer_checked. Will fail if CPI Guard is enabled
    /// on the sender's token account.
    #[instruction(discriminator = 0)]
    pub fn cpi_transfer(ctx: Ctx<CpiTransfer>) -> Result<(), ProgramError> {
        handle_cpi_transfer(&mut ctx.accounts)
    }
}

#[derive(Accounts)]
pub struct CpiTransfer<'info> {
    #[account(mut)]
    pub sender: &'info Signer,
    #[account(mut)]
    pub sender_token_account: &'info mut UncheckedAccount,
    pub mint_account: &'info UncheckedAccount,
    #[account(mut)]
    pub recipient_token_account: &'info mut UncheckedAccount,
    pub token_program: &'info Program<Token2022Program>,
}

#[inline(always)]
pub fn handle_cpi_transfer(accounts: &mut CpiTransfer) -> Result<(), ProgramError> {
    // TransferChecked: opcode 12, amount=1, decimals=9
    let mut data = [0u8; 10];
    data[0] = 12;
    data[1..9].copy_from_slice(&1u64.to_le_bytes());
    data[9] = 9; // decimals

    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [
            InstructionAccount::writable(accounts.sender_token_account.to_account_view().address()),
            InstructionAccount::readonly(accounts.mint_account.to_account_view().address()),
            InstructionAccount::writable(accounts.recipient_token_account.to_account_view().address()),
            InstructionAccount::readonly_signer(accounts.sender.to_account_view().address()),
        ],
        [
            accounts.sender_token_account.to_account_view(),
            accounts.mint_account.to_account_view(),
            accounts.recipient_token_account.to_account_view(),
            accounts.sender.to_account_view(),
        ],
        data,
    )
    .invoke()
}
