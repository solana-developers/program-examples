//! World Cup Solana Program.
//!
//! A bracket-prediction game starting at the Round of 32. Entrants pay a fixed fee
//! to submit a consistency-checked bracket; an admin oracle records results; a
//! permissionless, idempotent `refresh_score` folds each bracket into a provable
//! global tally; the unique winner claims the pot. The ranking key is total
//! (score, then goal-closeness, then earliest submission), so a winner always exists.
//!
//! Built on the [Pinocchio](https://docs.rs/pinocchio) runtime; uses
//! [Codama](https://github.com/codama-idl/codama) for IDL generation.

#![no_std]

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

use pinocchio::address::declare_id;

pub mod errors;
pub use errors::*;

pub mod event_engine;
pub mod events;

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod tournament;
pub use tournament::*;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

#[cfg(test)]
mod tests;

declare_id!("wCupoZtR1g1NXRRVELe5KqFgayyEteVKKxEerxugvxA");

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "World Cup Program",
    project_url: "https://github.com/solana-foundation/world-cup",
    contacts: "link:https://github.com/solana-foundation/world-cup/security/advisories/new",
    policy: "https://github.com/solana-foundation/world-cup/security/policy",
    source_code: "https://github.com/solana-foundation/world-cup"
}
