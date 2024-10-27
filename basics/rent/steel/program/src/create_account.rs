use rent_example_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_create_account(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    let args = CreateSystemAccount::try_from_bytes(data)?;
    
    let [payer_info, new_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    payer_info.is_signer()?;
    new_account_info.is_signer()?;
    
    let account_size = std::mem::size_of::<Address>();
    let rent = Rent::get()?;
    let lamports_required = rent.minimum_balance(account_size);

    msg!("Account size: {}", account_size);
    msg!("Lamports required: {}", lamports_required);

    // Create account with correct type argument
    create_account::<Address>(
        new_account_info,
        system_program,
        payer_info,
        &rent_example_api::ID,
        &[],
    )?;

    let address = new_account_info.as_account_mut::<Address>(&rent_example_api::ID)?;
    address.name = args.name;
    address.address = args.address;

    Ok(())
}
