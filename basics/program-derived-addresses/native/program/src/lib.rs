use solana_program::entrypoint;

use processor::process_instruction;

pub mod instructions;
pub mod processor;
pub mod state;

entrypoint!(process_instruction);
