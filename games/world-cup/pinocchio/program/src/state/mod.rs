//! On-chain account state types for the world-cup program.
//!
//! Each data-carrying account is stored as a packed C struct in a Program Derived
//! Account (PDA), with a one-byte discriminator at offset 0. The pot [`Vault`] is a
//! zero-data PDA.

pub mod bracket;
pub mod common;
pub mod config;
pub mod oracle;
pub mod vault;

pub use bracket::Bracket;
pub use common::{
    find_bracket_pda, find_config_pda, find_oracle_pda, find_vault_pda, verify_bracket_pda, AccountDiscriminator,
    TournamentState, ENTRY_FEE,
};
pub use config::Config;
pub use oracle::Oracle;
pub use vault::Vault;
