mod with_program;

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing instruction");
    example_function(program_id, accounts)?;
    Ok(())
}

pub fn example_function(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let seeds: &[&[u8]] = &[b"example_seed"];
    with_program::pda_rent_payer(program_id, accounts, seeds)
}

