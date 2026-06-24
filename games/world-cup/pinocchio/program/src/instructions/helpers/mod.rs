//! Account validation and PDA creation helpers.

use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    AccountView, ProgramResult,
};
use pinocchio_system::instructions::{Allocate, Assign, CreateAccount, Transfer};

use crate::WorldCupError;

/// Returns an error unless `account` is a transaction signer.
pub fn check_signer(account: &AccountView) -> Result<(), ProgramError> {
    if !account.is_signer() {
        return Err(WorldCupError::NotSigner.into());
    }
    Ok(())
}

/// Returns an error unless `account` is marked writable.
pub fn check_writable(account: &AccountView) -> Result<(), ProgramError> {
    if !account.is_writable() {
        return Err(WorldCupError::AccountNotWritable.into());
    }
    Ok(())
}

/// Returns an error unless `account` is the System Program.
pub fn check_system_program(account: &AccountView) -> Result<(), ProgramError> {
    if account.address().ne(&pinocchio_system::ID) {
        return Err(WorldCupError::NotSystemProgram.into());
    }
    Ok(())
}

/// Creates and allocates a program-owned PDA, funding rent from `payer`.
///
/// `seeds` must include the bump seed as the final element. Idempotent against a
/// pre-funded PDA address: tops up rent then allocates and assigns.
pub fn create_pda_account(payer: &AccountView, account: &AccountView, seeds: &[Seed], space: usize) -> ProgramResult {
    let lamports = Rent::get()?.try_minimum_balance(space)?;
    let signer = [Signer::from(seeds)];

    if account.lamports() == 0 {
        CreateAccount { from: payer, to: account, lamports, space: space as u64, owner: &crate::ID }
            .invoke_signed(&signer)?;
    } else {
        let required = lamports.saturating_sub(account.lamports());
        if required > 0 {
            Transfer { from: payer, to: account, lamports: required }.invoke()?;
        }
        Allocate { account, space: space as u64 }.invoke_signed(&signer)?;
        Assign { account, owner: &crate::ID }.invoke_signed(&signer)?;
    }

    Ok(())
}

/// Closes a program-owned `account`, transferring its rent lamports to `destination`,
/// zeroing its data, and reassigning it to the system program.
pub fn close_account(account: &AccountView, destination: &AccountView) -> ProgramResult {
    let mut account = *account;
    let mut destination = *destination;

    let new_balance =
        destination.lamports().checked_add(account.lamports()).ok_or(WorldCupError::ArithmeticOverflow)?;
    destination.set_lamports(new_balance);

    account.close()
}
