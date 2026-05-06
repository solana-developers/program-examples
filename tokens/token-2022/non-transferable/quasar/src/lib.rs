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

/// Creates a non-transferable mint (soulbound token). The NonTransferable
/// extension prevents any transfer of tokens minted from this mint.
#[program]
mod quasar_non_transferable {
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
    pub mint_account: &'info Signer,
    pub token_program: &'info Program<Token2022Program>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize(accounts: &Initialize) -> Result<(), ProgramError> {
    // Mint + NonTransferable extension = 170 bytes
    let mint_size: u64 = 170;
    let lamports = Rent::get()?.try_minimum_balance(mint_size as usize)?;

    // 1. Create account
    accounts.system_program
        .create_account(
            accounts.payer,
            accounts.mint_account,
            lamports,
            mint_size,
            accounts.token_program.to_account_view().address(),
        )
        .invoke()?;

    // 2. Initialize NonTransferable extension: opcode 32 (InitializeNonTransferableMint, no data)
    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [InstructionAccount::writable(
            accounts.mint_account.to_account_view().address(),
        )],
        [accounts.mint_account.to_account_view()],
        [32u8],
    )
    .invoke()?;

    // 3. InitializeMint2
    let mut mint_data = [0u8; 67];
    mint_data[0] = 20;
    mint_data[1] = 2; // decimals
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
