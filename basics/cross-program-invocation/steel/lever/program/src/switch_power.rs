use lever_api::prelude::*;
use steel::*;
use solana_program::*;
use crate::system_program::ID;


pub fn process_switch_power(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [power_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let args = SwitchPowerArgs::try_from_bytes(data)?;
    let name = std::str::from_utf8(&args.name[..args.name_len as usize])
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let power = power_info.to_account_mut::<PowerStatus>(&ID)?;
    power.is_on = if power.is_on == 0 { 1 } else { 0 };

    msg!("{} is pulling the power switch!", name);
    match power.is_on {
        1 => msg!("The power is now on."),
        0 => msg!("The power is now off!"),
        _ => unreachable!(),
    }

    Ok(())
}
