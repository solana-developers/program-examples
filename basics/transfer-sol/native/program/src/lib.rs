
pub mod instruction;
pub mod processor;

use {
    solana_program::entrypoint,
    crate::processor::process_instruction,
};

entrypoint!(process_instruction);