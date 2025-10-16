pub mod instruction;
pub mod processor;

use {crate::processor::process_instruction, solana_program::entrypoint};

entrypoint!(process_instruction);
