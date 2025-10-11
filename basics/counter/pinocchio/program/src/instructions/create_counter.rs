use crate::{load, require, states::Counter};
use {
    pinocchio::{
        account_info::AccountInfo,
        instruction::{Seed, Signer},
        program_error::ProgramError,
        pubkey::{find_program_address, pubkey_eq, Pubkey},
        sysvars::{rent::Rent, Sysvar},
        ProgramResult,
    },
    pinocchio_log::log,
    pinocchio_system::instructions::CreateAccount,
};
pub fn process_create_counter(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    log!("Counter Instruction: CreateCounter");
    validate(program_id, accounts)?;

    if let [creator, counter_pda, _system_program] = accounts {
        let counter_seeds = &[Counter::PREFIX];

        let (expected_counter, bump) = find_program_address(counter_seeds, program_id);

        require(
            pubkey_eq(&expected_counter, counter_pda.key()),
            ProgramError::IncorrectProgramId,
            "Validation Error: Seed constraints violated",
        )?;
        let curve_bump: &[u8] = &[bump];
        let seeds = [Seed::from(Counter::PREFIX), Seed::from(curve_bump)];
        let signer = Signer::from(&seeds);

        CreateAccount {
            from: creator,
            lamports: Rent::get()?.minimum_balance(Counter::SIZE),
            owner: program_id,
            space: Counter::SIZE as u64,
            to: counter_pda,
        }
        .invoke_signed(&[signer])?;

        let counter = load::<Counter>(counter_pda)?;

        counter.count = 0;

        Ok(())
    } else {
        Err(ProgramError::NotEnoughAccountKeys)
    }
}
pub fn validate(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    if let [creator, counter_pda, _system_program] = accounts {
        require(
            creator.is_signer(),
            ProgramError::MissingRequiredSignature,
            "Validation Error: Creator must be a signer",
        )?;

        require(
            counter_pda.is_writable(),
            ProgramError::InvalidAccountData,
            "Validation Error: Counter program Writable priviledge escalated",
        )?;

        Ok(())
    } else {
        Err(ProgramError::NotEnoughAccountKeys)
    }
}
