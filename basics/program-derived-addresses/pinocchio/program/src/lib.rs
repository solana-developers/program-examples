#![no_std]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use pinocchio_pubkey::declare_id;
mod processor;
use processor::process_instruction;
mod helper;
pub use helper::*;
mod instructions;
mod states;
declare_id!("8TpdLD58VBWsdzxRi2yRcmKJD9UcE2GuUrBwsyCwpbUN");

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();
