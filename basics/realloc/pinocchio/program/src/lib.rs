#![no_std]
pub mod instructions;
pub mod processor;
pub mod state;

use {
    crate::processor::process_instruction,
    pinocchio::{entrypoint, nostd_panic_handler},
};

entrypoint!(process_instruction);
nostd_panic_handler!();
