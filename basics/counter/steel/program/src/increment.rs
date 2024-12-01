use counter_api::prelude::*;
use steel::*;

use solana_program::msg;

pub fn process_increment(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    msg!("Processing Increment instruction");

    // Parse args.
    let args = Increment::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);
    msg!("Parsed amount: {}", amount);

    // Load accounts.
    let [signer_info, counter_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let counter = counter_info
        .as_account_mut::<Counter>(&counter_api::ID)?
        .assert_mut(|c| c.value < 100)?;

    // Update state
    counter.value += amount;

    msg!("Final amount: {}", counter.value);
    Ok(())
}
