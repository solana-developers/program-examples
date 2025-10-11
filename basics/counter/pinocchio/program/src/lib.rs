#![no_std]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use pinocchio_pubkey::declare_id;

pub mod helpers;
pub mod instructions;
pub mod processor;
pub mod states;
pub use helpers::*;
use processor::*;

program_entrypoint!(process_instructions);
nostd_panic_handler!();
no_allocator!();

declare_id!("8TpdLD58VBWsdzxRi2yRcmKJD9UcE2GuUrBwsyCwpbUN");
