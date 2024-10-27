use solana_program::entrypoint;

pub mod state;
pub mod instructions;
pub mod processor;

use processor::process_instruction;

entrypoint!(process_instruction);



