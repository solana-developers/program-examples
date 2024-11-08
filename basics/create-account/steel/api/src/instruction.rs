use solana_program::{msg, program::invoke, system_instruction};
use steel::*;
use sysvar::rent::Rent;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    CreateAccount = 0,
}

instruction!(SteelInstruction, CreateAccount);
// CreateAccount instruction
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateAccount {}

impl CreateAccount {
    pub fn process_instruction(accounts: &[AccountInfo]) -> ProgramResult {
        let [payer, new_account, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        msg!("Program invoked. Creating a system account...");
        msg!("  New public key will be: {}", &new_account.key.to_string());

        // get minimum lamports required for 0 data space
        //
        let lamports = Rent::get()?.minimum_balance(0);

        // use `create_account` or `allocate_account` steel helper for creating
        // pda accounts.
        //
        invoke(
            &system_instruction::create_account(
                payer.key,
                new_account.key,
                lamports,            // send lmaports
                0,                   // space
                &system_program::ID, // owner program
            ),
            &[payer.clone(), new_account.clone(), system_program.clone()],
        )?;

        msg!("Account created succesfully.");

        Ok(())
    }
}
