use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::state::{AddressInfo, EnhancedAddressInfo, EnhancedAddressInfoExtender, WorkInfo};

pub fn reallocate_without_zero_init(
    accounts: &[AccountInfo],
    args: EnhancedAddressInfoExtender,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let address_info_data = AddressInfo::try_from_slice(&target_account.data.borrow())?;
    let enhanced_address_info_data =
        EnhancedAddressInfo::from_address_info(address_info_data, args.state, args.zip);

    let account_span = (enhanced_address_info_data.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    let diff = lamports_required - target_account.lamports();
    invoke(
        &system_instruction::transfer(payer.key, target_account.key, diff),
        &[
            payer.clone(),
            target_account.clone(),
            system_program.clone(),
        ],
    )?;

    target_account.realloc(account_span, false)?;

    enhanced_address_info_data.serialize(&mut &mut target_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn reallocate_zero_init(accounts: &[AccountInfo], data: WorkInfo) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_info(accounts_iter)?;

    let account_span = (data.try_to_vec()?).len();

    target_account.realloc(account_span, true)?;

    data.serialize(&mut &mut target_account.data.borrow_mut()[..])?;

    Ok(())
}
