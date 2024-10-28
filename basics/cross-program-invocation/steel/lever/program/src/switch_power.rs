use lever_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_switch_power(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = SetPowerStatus::try_from_bytes(data)?;
    let name = bytes_to_str(&args.name);

    // Load accounts.
    let [power_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    power_info.is_writable()?;

    let power = power_info
        .as_account_mut::<PowerStatus>(&lever_api::ID)?
        .assert_mut(|c| c.is_on <= 1)?;

    match power.is_on {
        0 => power.is_on = 1,
        1 => power.is_on = 0,
        _ => panic!("Invalid boolean value"),
    }

    msg!("{} is pulling the power switch!", &name);

    match power.is_on {
        1 => msg!("The power is now on."),
        0 => msg!("The power is now off!"),
        _ => panic!("Invalid boolean value"),
    };

    Ok(())
}
