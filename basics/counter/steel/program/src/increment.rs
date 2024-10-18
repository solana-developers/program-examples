use steel_api::prelude::*;
use steel::*;
use solana_program::msg;

pub fn process_increment(accounts: &[AccountInfo<'_>]) -> ProgramResult {

    // Load accounts.
    let [signer_info, counter_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);        
    };
    signer_info.is_signer()?;
	let counter = counter_info
		.to_account_mut::<Counter>(&steel_api::ID)?
		.check_mut(|c| c.value < 100)?;

    // Update state 
	counter.value += 1;

    msg!("Increment");
    msg!("Counter Value {}", counter.value);

    Ok(())
}
