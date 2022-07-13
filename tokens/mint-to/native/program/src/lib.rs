use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult, 
    pubkey::Pubkey,
};

pub mod processor;
pub mod instructions;
pub mod state;


entrypoint!(process_instruction);


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    processor::process_instruction(accounts, instruction_data)
}