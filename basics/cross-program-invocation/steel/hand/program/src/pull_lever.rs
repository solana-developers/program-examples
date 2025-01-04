use hand_api::prelude::*;
use lever_api::prelude::*;
use steel::*;

pub fn process_pull_lever(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = PullLever::try_from_bytes(data)?;
    let name = bytes_to_str(&args.name);

    // Load accounts.
    let [power_info, lever_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    power_info.is_writable()?;

    let ix = switch_power(*power_info.key, &name);

    solana_program::program::invoke(&ix, &[power_info.clone(), lever_program.clone()])?;

    Ok(())
}
