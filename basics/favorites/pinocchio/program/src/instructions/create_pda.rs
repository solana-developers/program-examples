use crate::state::Favorites;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use pinocchio_system::instructions::CreateAccount;

pub fn create_pda(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [user, favorite_account, _] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // deriving the favorite pda
    let (favorite_pda, favorite_bump) =
        find_program_address(&[b"favorite", user.key().as_ref()], program_id);

    // Checking if the favorite account is same as the derived favorite pda
    if favorite_account.key() != &favorite_pda {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Checking if the pda is already initialized
    if favorite_account.try_borrow_data()?.len() == 0 {
        let rent = Rent::get()?;

        // Initialize the favorite account if it's not initialized
        let space = size_of::<Favorites>();
        let lamports = rent.minimum_balance(space);

        let bump_bytes = favorite_bump.to_le_bytes();

        let seeds = [
            Seed::from(b"favorite"),
            Seed::from(user.key()),
            Seed::from(&bump_bytes),
        ];

        let signers = [Signer::from(&seeds)];

        CreateAccount {
            from: user,
            to: favorite_account,
            lamports,
            space: space as u64,
            owner: program_id,
        }
        .invoke_signed(&signers)?;

        // Serialize and store the data
        let mut favrite_account_data = favorite_account.try_borrow_mut_data()?;
        favrite_account_data.copy_from_slice(data);
    } else {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    Ok(())
}
