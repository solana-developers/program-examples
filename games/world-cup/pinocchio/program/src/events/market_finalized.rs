use core::mem::size_of;

use alloc::vec::Vec;
use pinocchio::Address;

use crate::event_engine::{EventDiscriminator, EventDiscriminators, EventSerialize};

/// Emitted when the admin finalizes a unique winner.
#[repr(C, packed)]
pub struct MarketFinalizedEvent {
    pub winner: Address,
    pub best_score: u16,
    pub best_closeness: u16,
}

impl MarketFinalizedEvent {
    pub const DATA_LEN: usize = size_of::<Self>();

    pub fn new(winner: Address, best_score: u16, best_closeness: u16) -> Self {
        Self { winner, best_score, best_closeness }
    }
}

impl EventDiscriminator for MarketFinalizedEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::MarketFinalized as u8;
}

impl EventSerialize for MarketFinalizedEvent {
    const DATA_LEN: usize = Self::DATA_LEN;

    fn write_inner(&self, writer: &mut Vec<u8>) {
        let best_score = self.best_score;
        let best_closeness = self.best_closeness;
        writer.extend_from_slice(self.winner.as_ref());
        writer.extend_from_slice(&best_score.to_le_bytes());
        writer.extend_from_slice(&best_closeness.to_le_bytes());
    }
}
