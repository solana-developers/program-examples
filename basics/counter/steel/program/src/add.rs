
use steelcounter_api::prelude::*;
use steel::*;
use solana_program::msg;

pub fn process_add(accounts: &[AccountInfo<'_>]) -> ProgramResult {
    // Parse args.
    // let args = Add::try_from_bytes()?;
    // let amount = u64::from_le_bytes(args.amount);
    // Load accounts.
    let [signer_info, counter_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let counter = counter_info
        .to_account_mut::<Counter>(&steelcounter_api::ID)?
        .check_mut(|c| c.value < 100)?;

    // Update state
	counter.value += 1;

    msg!("Counter value incremented!");
    msg!("Counter value: {:?}", counter);
    Ok(())
}
