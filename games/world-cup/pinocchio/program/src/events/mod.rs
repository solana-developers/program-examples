//! Event types emitted by the world-cup program via self-CPI.
//!
//! Each event struct implements [`EventDiscriminator`](crate::event_engine::EventDiscriminator)
//! and [`EventSerialize`](crate::event_engine::EventSerialize): an 8-byte tag prefix,
//! a 1-byte discriminator, then the event-specific payload.

pub mod bracket_closed;
pub mod bracket_submitted;
pub mod config_initialized;
pub mod goals_posted;
pub mod market_finalized;
pub mod pot_claimed;
pub mod result_posted;
pub mod score_refreshed;
pub mod tournament_locked;

pub use bracket_closed::BracketClosedEvent;
pub use bracket_submitted::BracketSubmittedEvent;
pub use config_initialized::ConfigInitializedEvent;
pub use goals_posted::GoalsPostedEvent;
pub use market_finalized::MarketFinalizedEvent;
pub use pot_claimed::PotClaimedEvent;
pub use result_posted::ResultPostedEvent;
pub use score_refreshed::ScoreRefreshedEvent;
pub use tournament_locked::TournamentLockedEvent;
