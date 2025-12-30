#![no_std]
use pinocchio::{entrypoint, nostd_panic_handler};

use processor::process_instruction;

pub mod instructions;
pub mod processor;
pub mod state;

entrypoint!(process_instruction);
nostd_panic_handler!();
