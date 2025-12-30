use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

use crate::state::RentVault;

pub struct InitRentVaultArgs {
    pub fund_lamports: u64,
}

pub fn init_rent_vault(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [rent_vault, payer, _] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let (rent_vault_pda, rent_vault_bump) =
        find_program_address(&[RentVault::SEED_PREFIX.as_bytes()], program_id);
    assert!(rent_vault.key().eq(&rent_vault_pda));

    // Lamports for rent on the vault, plus the desired additional funding
    //
    let fund_lamports = u64::from_le_bytes(
        instruction_data[0..8]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );
    let lamports_required = (Rent::get()?).minimum_balance(0) + fund_lamports;

    let bump_bytes = rent_vault_bump.to_le_bytes();

    let seeds = [
        Seed::from(RentVault::SEED_PREFIX.as_bytes()),
        Seed::from(&bump_bytes),
    ];

    let signer_seed = Signer::from(&seeds);

    CreateAccount {
        from: payer,
        to: rent_vault,
        lamports: lamports_required,
        space: 0,
        owner: program_id,
    }
    .invoke_signed(&[signer_seed])?;

    Ok(())
}
