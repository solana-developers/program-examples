use crate::{state::RentVault, SteelInstruction};
use steel::*;

instruction!(SteelInstruction, InitRentVault);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InitRentVault {
    pub fund_lamports: u64,
}

impl InitRentVault {
    pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        // Parse args.
        let args = InitRentVault::try_from_bytes(data)?;

        let [rent_vault, payer, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        payer.is_signer()?;

        rent_vault
            .is_writable()?
            .has_owner(&system_program::ID)? // we check that the account is owned by the system program.
            .has_seeds(&[RentVault::SEED_PREFIX], &crate::ID)?;

        system_program.is_program(&system_program::ID)?;

        // create rent vault lamports to rent vault
        create_account::<RentVault>(
            rent_vault,
            system_program,
            payer,
            &crate::ID,
            &[RentVault::SEED_PREFIX],
        )?;

        // send funds to vault
        rent_vault.collect(args.fund_lamports, payer)
    }
}
