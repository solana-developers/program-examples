use borsh::{BorshDeserialize, BorshSerialize};
#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Ok(power_status) = PowerStatus::try_from_slice(instruction_data) {
        return initialize(program_id, accounts, power_status);
    }

    if let Ok(set_power_status) = SetPowerStatus::try_from_slice(instruction_data) {
        return switch_power(accounts, set_power_status.name);
    }

    Err(ProgramError::InvalidInstructionData)
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    power_status: PowerStatus,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_span = (power_status.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    invoke(
        &system_instruction::create_account(
            user.key,
            power.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[user.clone(), power.clone(), system_program.clone()],
    )?;

    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    Ok(())
}

pub fn switch_power(accounts: &[AccountInfo], name: String) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;

    let mut power_status = PowerStatus::try_from_slice(&power.data.borrow())?;
    power_status.is_on = !power_status.is_on;
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    msg!("{} is pulling the power switch!", &name);

    match power_status.is_on {
        true => msg!("The power is now on."),
        false => msg!("The power is now off!"),
    };

    Ok(())
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SetPowerStatus {
    pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PowerStatus {
    pub is_on: bool,
}
