#![no_std]
#![allow(deprecated)]

use pinocchio::{
    account_info::AccountInfo,
    entrypoint, nostd_panic_handler,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::rent::{
        Rent, DEFAULT_BURN_PERCENT, DEFAULT_EXEMPTION_THRESHOLD, DEFAULT_LAMPORTS_PER_BYTE_YEAR,
    },
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

entrypoint!(process_instruction);
nostd_panic_handler!();

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [payer, new_account, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    log!("Program invoked. Creating a system account...");
    log!("  New public key will be: ");
    pinocchio::pubkey::log(&new_account.key());

    let rent = Rent {
        lamports_per_byte_year: DEFAULT_LAMPORTS_PER_BYTE_YEAR,
        exemption_threshold: DEFAULT_EXEMPTION_THRESHOLD,
        burn_percent: DEFAULT_BURN_PERCENT,
    };

    // Determine the necessary minimum rent by calculating the account's size
    //
    let account_span = instruction_data.len();
    let lamports_required = rent.minimum_balance(account_span);

    log!(50, "Account span: {}", account_span);
    log!(50, "Lamports required: {}", lamports_required);

    CreateAccount {
        from: payer,
        to: new_account,
        lamports: lamports_required,
        space: account_span as u64,
        owner: program_id,
    }
    .invoke()?;

    log!("Account created succesfully.");
    Ok(())
}
