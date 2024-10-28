use lever_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_switch_power(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = SwitchPower::try_from_bytes(data)?;
    let name = String::from_utf8_lossy(&args.name[..])
        .trim_end_matches('\0')
        .to_string();

    msg!("{} is pulling the power switch!", &name);

    // Load accounts.
    let [power_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    power_info.is_writable()?;

    let power = power_info
        .as_account_mut::<PowerStatus>(&lever_api::ID)?
        .assert_mut(|c| c.is_on <= 1)?;

    // Update state
    match power.is_on {
        0 => power.is_on = 1,
        1 => power.is_on = 0,
        _ => panic!("Must be a boolean value of 0 or 1"),
    }

    match power.is_on {
        1 => msg!("The power is now on."),
        0 => msg!("The power is now off!"),
        _ => panic!("Invalid boolean value"),
    };

    Ok(())
}
