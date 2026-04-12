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

/// Creates a mint with the GroupPointer extension.
///
/// The Token Group and Token Member extensions are not yet fully enabled on
/// the Token-2022 program. This example demonstrates initializing the
/// GroupPointer extension on a mint. Actual group/member initialization
/// is commented out in the Anchor version as well.
#[program]
mod quasar_group {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize_group(ctx: Ctx<InitializeGroup>) -> Result<(), ProgramError> {
        handle_initialize(&mut ctx.accounts)
    }
}

#[derive(Accounts)]
pub struct InitializeGroup<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    #[account(mut)]
    pub mint_account: &'info Signer,
    pub token_program: &'info Program<Token2022Program>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize(accounts: &InitializeGroup) -> Result<(), ProgramError> {
    // Mint + GroupPointer extension = 250 bytes
    let mint_size: u64 = 250;
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

    // InitializeGroupPointer: opcode 41, sub-opcode 0
    // Data: [41, 0, authority (32 bytes), group_address (32 bytes)]
    let mut ext_data = [0u8; 66];
    ext_data[0] = 41;
    ext_data[1] = 0;
    // authority = mint itself (self-referential PDA pattern)
    ext_data[2..34].copy_from_slice(accounts.mint_account.to_account_view().address().as_ref());
    // group_address = mint itself
    ext_data[34..66].copy_from_slice(accounts.mint_account.to_account_view().address().as_ref());

    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [InstructionAccount::writable(
            accounts.mint_account.to_account_view().address(),
        )],
        [accounts.mint_account.to_account_view()],
        ext_data,
    )
    .invoke()?;

    // InitializeMint2: mint authority = mint itself (for self-signing)
    let mut mint_data = [0u8; 67];
    mint_data[0] = 20;
    mint_data[1] = 2;
    mint_data[2..34].copy_from_slice(accounts.mint_account.to_account_view().address().as_ref());
    mint_data[34] = 1;
    mint_data[35..67].copy_from_slice(accounts.mint_account.to_account_view().address().as_ref());

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
