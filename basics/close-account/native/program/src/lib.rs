pub mod instructions;
pub mod processor;
pub mod state;

use {crate::processor::process_instruction, solana_program::entrypoint};

entrypoint!(process_instruction);
