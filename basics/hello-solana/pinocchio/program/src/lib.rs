use pinocchio::{
    account_info::AccountInfo,
    default_panic_handler, no_allocator, program_entrypoint,
    pubkey::{self, Pubkey},
    ProgramResult,
};
use pinocchio_log::log;

// This is the program entrypoint.
program_entrypoint!(process_instruction);
// Do not allocate memory.
no_allocator!();
// Use the default panic handler.
default_panic_handler!();

#[inline(always)]
fn process_instruction(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    log!("Hello, Solana!");

    pubkey::log(program_id);

    Ok(())
}
