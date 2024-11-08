use crate::{state::RentVault, SteelInstruction};
use solana_program::rent::Rent;
use steel::*;

instruction!(SteelInstruction, CreateNewAccount);

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateNewAccount {}

impl CreateNewAccount {
    pub fn process(accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [new_account, rent_vault, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        new_account.is_signer()?;
        rent_vault
            .is_writable()?
            .has_seeds(&[RentVault::SEED_PREFIX], &crate::ID)?;
        system_program.is_program(&system_program::ID)?;

        let lamports_required_for_rent = (Rent::get()?).minimum_balance(0);

        rent_vault.send(lamports_required_for_rent, new_account);

        Ok(())
    }
}
