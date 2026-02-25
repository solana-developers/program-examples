#![allow(deprecated)]
#![no_std]
#[cfg(not(feature = "no-entrypoint"))]
use pinocchio::entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

use pinocchio::{
    account_info::AccountInfo,
    nostd_panic_handler,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::rent::{
        Rent, DEFAULT_BURN_PERCENT, DEFAULT_EXEMPTION_THRESHOLD, DEFAULT_LAMPORTS_PER_BYTE_YEAR,
    },
    ProgramResult,
};

use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

nostd_panic_handler!();

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let _ = match instruction_data.split_first() {
        Some((0, instruction_data)) => initialize(program_id, accounts, instruction_data),
        Some((1, instruction_data)) => switch_power(accounts, instruction_data),
        _ => return Err(ProgramError::InvalidInstructionData),
    };
    Ok(())
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [power, user, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let rent = Rent {
        lamports_per_byte_year: DEFAULT_LAMPORTS_PER_BYTE_YEAR,
        exemption_threshold: DEFAULT_EXEMPTION_THRESHOLD,
        burn_percent: DEFAULT_BURN_PERCENT,
    };

    let account_span = size_of::<PowerStatus>();
    let lamports_required = rent.minimum_balance(account_span);

    CreateAccount {
        from: user,
        to: power,
        lamports: lamports_required,
        space: account_span as u64,
        owner: program_id,
    }
    .invoke()?;

    let mut power_status_data = power
        .try_borrow_mut_data()
        .map_err(|_| ProgramError::InvalidAccountData)?;

    power_status_data.copy_from_slice(instruction_data);

    Ok(())
}

pub fn switch_power(accounts: &[AccountInfo], _instruction_data: &[u8]) -> ProgramResult {
    let [power] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let mut power_status = power.try_borrow_mut_data()?;

    let power_status_bytes: [u8; 1] = power_status[0..1]
        .try_into()
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let mut is_on = u8::from_le_bytes(power_status_bytes);
    match is_on {
        0 => is_on = 1,
        1 => is_on = 0,
        _ => return Err(ProgramError::InvalidAccountData),
    }

    power_status.copy_from_slice(&is_on.to_le_bytes());

    match is_on {
        1 => {
            log!("The power is now on.");
        }
        0 => {
            log!("The power is now off!");
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };

    Ok(())
}

pub struct SetPowerStatus {
    pub name: [u8],
}

pub struct PowerStatus {
    pub is_on: u8,
}
