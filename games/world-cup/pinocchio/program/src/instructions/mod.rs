//! Instruction definitions and dispatch for the world-cup program.
//!
//! Each instruction variant carries its own discriminator (the first byte of
//! instruction data). The Codama annotations on each variant describe the required
//! accounts in positional order.

pub mod claim;
pub mod close_bracket;
pub mod emit_event;
pub mod finalize;
pub mod helpers;
pub mod init_config;
pub mod lock;
pub mod post_goals;
pub mod post_result;
pub mod refresh_score;
pub mod submit_bracket;

pub use helpers::*;
pub use init_config::InitConfigData;
pub use post_goals::PostGoalsData;
pub use post_result::PostResultData;
pub use submit_bracket::SubmitBracketData;

use core::fmt;

use codama::CodamaInstructions;
use pinocchio::error::ProgramError;

use crate::event_engine::EMIT_EVENT_IX_DISC;
use crate::WorldCupError;

/// All instructions supported by the world-cup program.
#[derive(Debug, CodamaInstructions)]
#[repr(u8)]
#[allow(clippy::large_enum_variant)]
pub enum WorldCupInstruction {
    #[codama(account(name = "admin", signer, writable, docs = "Tournament admin; funds and owns the config"))]
    #[codama(account(name = "config", writable, docs = "Singleton config PDA", default_value = pda("config", [])))]
    #[codama(account(name = "oracle", writable, docs = "Singleton oracle PDA", default_value = pda("oracle", [])))]
    #[codama(account(name = "vault", writable, docs = "Pot vault PDA", default_value = pda("vault", [])))]
    #[codama(account(name = "system_program", docs = "The system program", default_value = program("system")))]
    #[codama(account(name = "event_authority", docs = "The event authority PDA", default_value = pda("event_authority", [])))]
    #[codama(account(
        name = "self_program",
        docs = "This program (for self-CPI event emission)",
        default_value = public_key("wCupoZtR1g1NXRRVELe5KqFgayyEteVKKxEerxugvxA")
    ))]
    InitConfig(#[codama(name = "init_config_data")] InitConfigData) = 0,

    #[codama(account(name = "entrant", signer, writable, docs = "The entrant submitting and paying for a bracket"))]
    #[codama(account(name = "config", writable, docs = "Singleton config PDA", default_value = pda("config", [])))]
    #[codama(account(
        name = "bracket",
        writable,
        docs = "The bracket PDA being created",
        default_value = pda("bracket", [seed("owner", account("entrant"))])
    ))]
    #[codama(account(name = "vault", writable, docs = "Pot vault PDA", default_value = pda("vault", [])))]
    #[codama(account(name = "system_program", docs = "The system program", default_value = program("system")))]
    #[codama(account(name = "event_authority", docs = "The event authority PDA", default_value = pda("event_authority", [])))]
    #[codama(account(
        name = "self_program",
        docs = "This program (for self-CPI event emission)",
        default_value = public_key("wCupoZtR1g1NXRRVELe5KqFgayyEteVKKxEerxugvxA")
    ))]
    SubmitBracket(#[codama(name = "submit_bracket_data")] SubmitBracketData) = 1,

    #[codama(account(name = "admin", signer, docs = "Tournament admin"))]
    #[codama(account(name = "config", writable, docs = "Singleton config PDA", default_value = pda("config", [])))]
    #[codama(account(name = "event_authority", docs = "The event authority PDA", default_value = pda("event_authority", [])))]
    #[codama(account(
        name = "self_program",
        docs = "This program (for self-CPI event emission)",
        default_value = public_key("wCupoZtR1g1NXRRVELe5KqFgayyEteVKKxEerxugvxA")
    ))]
    Lock = 2,

    #[codama(account(name = "admin", signer, docs = "Tournament admin (oracle)"))]
    #[codama(account(name = "config", docs = "Singleton config PDA", default_value = pda("config", [])))]
    #[codama(account(name = "oracle", writable, docs = "Singleton oracle PDA", default_value = pda("oracle", [])))]
    #[codama(account(name = "event_authority", docs = "The event authority PDA", default_value = pda("event_authority", [])))]
    #[codama(account(
        name = "self_program",
        docs = "This program (for self-CPI event emission)",
        default_value = public_key("wCupoZtR1g1NXRRVELe5KqFgayyEteVKKxEerxugvxA")
    ))]
    PostResult(#[codama(name = "post_result_data")] PostResultData) = 3,

    #[codama(account(name = "admin", signer, docs = "Tournament admin (oracle)"))]
    #[codama(account(name = "config", docs = "Singleton config PDA", default_value = pda("config", [])))]
    #[codama(account(name = "oracle", writable, docs = "Singleton oracle PDA", default_value = pda("oracle", [])))]
    #[codama(account(name = "event_authority", docs = "The event authority PDA", default_value = pda("event_authority", [])))]
    #[codama(account(
        name = "self_program",
        docs = "This program (for self-CPI event emission)",
        default_value = public_key("wCupoZtR1g1NXRRVELe5KqFgayyEteVKKxEerxugvxA")
    ))]
    PostGoals(#[codama(name = "post_goals_data")] PostGoalsData) = 4,

    #[codama(account(name = "config", writable, docs = "Singleton config PDA", default_value = pda("config", [])))]
    #[codama(account(name = "oracle", docs = "Singleton oracle PDA", default_value = pda("oracle", [])))]
    #[codama(account(name = "bracket", writable, docs = "The bracket PDA being scored"))]
    #[codama(account(name = "event_authority", docs = "The event authority PDA", default_value = pda("event_authority", [])))]
    #[codama(account(
        name = "self_program",
        docs = "This program (for self-CPI event emission)",
        default_value = public_key("wCupoZtR1g1NXRRVELe5KqFgayyEteVKKxEerxugvxA")
    ))]
    RefreshScore = 5,

    #[codama(account(name = "admin", signer, writable, docs = "Tournament admin"))]
    #[codama(account(name = "config", writable, docs = "Singleton config PDA", default_value = pda("config", [])))]
    #[codama(account(name = "oracle", docs = "Singleton oracle PDA", default_value = pda("oracle", [])))]
    #[codama(account(name = "bracket", docs = "The bracket claimed to be the unique winner"))]
    #[codama(account(name = "event_authority", docs = "The event authority PDA", default_value = pda("event_authority", [])))]
    #[codama(account(
        name = "self_program",
        docs = "This program (for self-CPI event emission)",
        default_value = public_key("wCupoZtR1g1NXRRVELe5KqFgayyEteVKKxEerxugvxA")
    ))]
    Finalize = 6,

    #[codama(account(name = "winner", signer, writable, docs = "The recorded winner claiming the pot"))]
    #[codama(account(name = "config", writable, docs = "Singleton config PDA", default_value = pda("config", [])))]
    #[codama(account(name = "vault", writable, docs = "Pot vault PDA", default_value = pda("vault", [])))]
    #[codama(account(name = "event_authority", docs = "The event authority PDA", default_value = pda("event_authority", [])))]
    #[codama(account(
        name = "self_program",
        docs = "This program (for self-CPI event emission)",
        default_value = public_key("wCupoZtR1g1NXRRVELe5KqFgayyEteVKKxEerxugvxA")
    ))]
    Claim = 7,

    #[codama(account(name = "config", docs = "Singleton config PDA", default_value = pda("config", [])))]
    #[codama(account(name = "bracket", writable, docs = "The bracket PDA being closed"))]
    #[codama(account(name = "vault", writable, docs = "Pot vault PDA", default_value = pda("vault", [])))]
    #[codama(account(name = "event_authority", docs = "The event authority PDA", default_value = pda("event_authority", [])))]
    #[codama(account(
        name = "self_program",
        docs = "This program (for self-CPI event emission)",
        default_value = public_key("wCupoZtR1g1NXRRVELe5KqFgayyEteVKKxEerxugvxA")
    ))]
    CloseBracket = 8,

    #[codama(skip)]
    #[codama(account(name = "event_authority", signer, docs = "The event authority PDA"))]
    EmitEvent = 228,
}

impl WorldCupInstruction {
    /// Parse a `WorldCupInstruction` from raw instruction bytes.
    pub fn from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        let (discriminator, rest) = data.split_first().ok_or(WorldCupError::InvalidInstruction)?;

        match discriminator {
            init_config::DISCRIMINATOR => Ok(Self::InitConfig(InitConfigData::load(rest)?.clone())),
            submit_bracket::DISCRIMINATOR => Ok(Self::SubmitBracket(SubmitBracketData::load(rest)?.clone())),
            lock::DISCRIMINATOR => Ok(Self::Lock),
            post_result::DISCRIMINATOR => Ok(Self::PostResult(PostResultData::load(rest)?.clone())),
            post_goals::DISCRIMINATOR => Ok(Self::PostGoals(PostGoalsData::load(rest)?.clone())),
            refresh_score::DISCRIMINATOR => Ok(Self::RefreshScore),
            finalize::DISCRIMINATOR => Ok(Self::Finalize),
            claim::DISCRIMINATOR => Ok(Self::Claim),
            close_bracket::DISCRIMINATOR => Ok(Self::CloseBracket),
            &EMIT_EVENT_IX_DISC => Ok(Self::EmitEvent),
            _ => Err(WorldCupError::InvalidInstruction.into()),
        }
    }
}

impl fmt::Display for WorldCupInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitConfig(_) => write!(f, "init_config"),
            Self::SubmitBracket(_) => write!(f, "submit_bracket"),
            Self::Lock => write!(f, "lock"),
            Self::PostResult(_) => write!(f, "post_result"),
            Self::PostGoals(_) => write!(f, "post_goals"),
            Self::RefreshScore => write!(f, "refresh_score"),
            Self::Finalize => write!(f, "finalize"),
            Self::Claim => write!(f, "claim"),
            Self::CloseBracket => write!(f, "close_bracket"),
            Self::EmitEvent => write!(f, "emit_event"),
        }
    }
}
