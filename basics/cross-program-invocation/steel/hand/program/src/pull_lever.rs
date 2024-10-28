use hand_api::prelude::*;
use lever_api::prelude::*;
use steel::*;

pub fn process_pull_lever(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    let args = PullLever::try_from_bytes(data)?;
    let name = String::from_utf8_lossy(&args.name[..])
        .trim_end_matches('\0')
        .to_string();

    // Load accounts.
    let [power_info, lever_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    power_info.is_writable()?;

    let ix = switch_power(*power_info.key, &name);

    solana_program::program::invoke(&ix, &[power_info.clone(), lever_program.clone()])?;

    Ok(())
}
