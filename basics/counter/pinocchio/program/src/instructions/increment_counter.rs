use crate::{load, require, states::Counter};
use {
    pinocchio::{
        account_info::AccountInfo,
        program_error::ProgramError,
        pubkey::{find_program_address, pubkey_eq, Pubkey},
        ProgramResult,
    },
    pinocchio_log::log,
};

pub fn process_increment_counter(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    log!("Counter Instruction: IncrementCounter");
    validate(program_id, accounts)?;

    if let [_user, counter_pda] = accounts {
        let counter = load::<Counter>(counter_pda)?;

        counter.count += 1;
        Ok(())
    } else {
        Err(ProgramError::NotEnoughAccountKeys)
    }
}

pub fn validate(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    if let [user, counter_pda] = accounts {
        require(
            user.is_signer(),
            ProgramError::MissingRequiredSignature,
            "Validation Error: User must be a signer",
        )?;

        let counter_seeds = &[Counter::PREFIX];

        let (expected_counter, _) = find_program_address(counter_seeds, program_id);

        require(
            pubkey_eq(&expected_counter, counter_pda.key()),
            ProgramError::IncorrectProgramId,
            "Validation Error: Seed constraints violated",
        )?;

        require(
            counter_pda.data_len() == Counter::SIZE,
            ProgramError::UninitializedAccount,
            "Validation Error: Counter isn't initialized yet",
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
