use steel_api::prelude::*;
use steel::*;

pub fn process_add(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Add::try_from_bytes(data)?;
	let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, counter_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);        
    };
    signer_info.is_signer()?;
	let counter = counter_info
		.as_account_mut::<Counter>(&steel_api::ID)?
		.assert_mut(|c| c.value < 100)?;

    // Update state 
	counter.value += amount;

    Ok(())
}
