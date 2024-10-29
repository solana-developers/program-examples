use hand_api::prelude::*;
use steel::*;

pub fn process_pull_lever(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [power_info, lever_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate lever program
    lever_program.is_program(&lever_api::ID)?;

    // Parse name from args
    let args = PullLeverArgs::try_from_bytes(data)?;
    let name = std::str::from_utf8(&args.name[..args.name_len as usize])
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Perform CPI to lever program
    solana_program::program::invoke(
        &lever_api::sdk::switch_power(
            *power_info.key,
            name.to_string(),
        ),
        &[
            power_info.clone(),
            lever_program.clone(),
        ],
    )?;

    Ok(())
}