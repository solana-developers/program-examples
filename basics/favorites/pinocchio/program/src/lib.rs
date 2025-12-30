#![no_std]
use pinocchio::entrypoint;

pub mod instructions;
pub mod processor;
pub mod state;

use processor::process_instruction;

pinocchio::nostd_panic_handler!();
entrypoint!(process_instruction);
