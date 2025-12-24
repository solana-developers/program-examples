use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::state::RentVault;

pub fn create_new_account(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [new_account, rent_vault, _] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let (rent_vault_pda, _rent_vault_bump) =
        find_program_address(&[RentVault::SEED_PREFIX.as_bytes()], program_id);
    assert!(rent_vault.key().eq(&rent_vault_pda));

    // Assuming this account has no inner data (size 0)
    //
    let lamports_required_for_rent = (Rent::get()?).minimum_balance(0);

    *rent_vault.try_borrow_mut_lamports()? -= lamports_required_for_rent;
    *new_account.try_borrow_mut_lamports()? += lamports_required_for_rent;

    Ok(())
}
