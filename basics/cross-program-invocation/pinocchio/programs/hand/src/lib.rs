#![no_std]

use pinocchio::{
    account_info::AccountInfo, entrypoint, instruction::AccountMeta, instruction::Instruction,
    nostd_panic_handler, program::invoke, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

entrypoint!(pull_lever);
nostd_panic_handler!();

fn pull_lever(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [power, lever_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let ix = Instruction {
        program_id: lever_program.key(), // Our lever program's ID
        data: instruction_data,          // Passing instructions through
        accounts: &[AccountMeta::new(&power.key(), true, false)], // Just the required account for the other program
    };

    invoke(&ix, &[&power.clone()])
}
