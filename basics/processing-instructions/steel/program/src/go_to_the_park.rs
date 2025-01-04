use processing_instructions_api::prelude::*;
use steel::*;

use solana_program::msg;

pub fn process_go_to_the_park(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    msg!("Processing GoToThePark instruction");

    // Parse args.
    let args = GoToThePark::try_from_bytes(data)?;

    // Load accounts.
    let [signer_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;

    msg!("Welcome to the park, {}!", args.data.name());

    if args.data.height() > 5 {
        msg!("You are tall enough to ride this ride. Congratulations.");
    } else {
        msg!("You are NOT tall enough to ride this ride. Sorry mate.");
    };

    Ok(())
}
