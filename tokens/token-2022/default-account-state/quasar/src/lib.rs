#![cfg_attr(not(test), no_std)]

use quasar_lang::sysvars::Sysvar;
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

/// Creates a mint with the DefaultAccountState extension (frozen by default),
/// and allows updating the default state.
#[program]
mod quasar_default_account_state {
    use super::*;

    /// Create a new mint with DefaultAccountState extension set to frozen.
    /// The mint account must be a signer (keypair created client-side).
    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>) -> Result<(), ProgramError> {
        handle_initialize(&mut ctx.accounts)
    }

    /// Update the default account state on an existing mint.
    /// 0 = Uninitialized, 1 = Initialized, 2 = Frozen
    #[instruction(discriminator = 1)]
    pub fn update_default_state(
        ctx: Ctx<UpdateDefaultState>,
        account_state: u8,
    ) -> Result<(), ProgramError> {
        handle_update_default_state(&mut ctx.accounts, account_state)
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
    // Mint + DefaultAccountState extension = 234 bytes
    let mint_size: u64 = 234;
    let lamports = Rent::get()?.try_minimum_balance(mint_size as usize)?;

    // 1. Create account owned by Token-2022
    accounts.system_program
        .create_account(
            accounts.payer,
            accounts.mint_account,
            lamports,
            mint_size,
            accounts.token_program.to_account_view().address(),
        )
        .invoke()?;

    // 2. Initialize DefaultAccountState extension (frozen = 2)
    // Instruction: ExtensionInstruction(DefaultAccountStateInitialize) = [28, 0, 2]
    let ext_data: [u8; 3] = [28, 0, 2]; // opcode 28, sub-opcode 0, state = Frozen
    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [InstructionAccount::writable(
            accounts.mint_account.to_account_view().address(),
        )],
        [accounts.mint_account.to_account_view()],
        ext_data,
    )
    .invoke()?;

    // 3. InitializeMint2: opcode 20, decimals, mint_authority, freeze_authority_option, freeze_authority
    // COption<Pubkey> is encoded as 4-byte little-endian tag (1 = Some, 0 = None) + 32-byte pubkey
    // Total: 1 (opcode) + 1 (decimals) + 32 (mint_authority) + 4 (COption tag) + 32 (freeze_authority) = 70 bytes
    let mut mint_data = [0u8; 70];
    mint_data[0] = 20; // InitializeMint2
    mint_data[1] = 2; // decimals
    mint_data[2..34].copy_from_slice(accounts.payer.to_account_view().address().as_ref());
    mint_data[34] = 1; // COption::Some discriminant (4-byte little-endian u32 = [1, 0, 0, 0])
    // mint_data[35..38] = [0, 0, 0] — already zeroed, completing 4-byte COption tag
    mint_data[38..70].copy_from_slice(accounts.payer.to_account_view().address().as_ref());

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
pub struct UpdateDefaultState<'info> {
    #[account(mut)]
    pub freeze_authority: &'info Signer,
    #[account(mut)]
    pub mint_account: &'info mut UncheckedAccount,
    pub token_program: &'info Program<Token2022Program>,
}

#[inline(always)]
pub fn handle_update_default_state(accounts: &UpdateDefaultState, account_state: u8) -> Result<(), ProgramError> {
    // DefaultAccountState Update: opcode 28, sub-opcode 1, new state
    let data: [u8; 3] = [28, 1, account_state];
    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [
            InstructionAccount::writable(accounts.mint_account.to_account_view().address()),
            InstructionAccount::readonly_signer(
                accounts.freeze_authority.to_account_view().address(),
            ),
        ],
        [
            accounts.mint_account.to_account_view(),
            accounts.freeze_authority.to_account_view(),
        ],
        data,
    )
    .invoke()
}
