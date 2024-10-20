use processor::process_instruction;
use solana_program::entrypoint;
pub mod instructions;
pub mod processor;
pub mod state;

entrypoint!(process_instruction);
