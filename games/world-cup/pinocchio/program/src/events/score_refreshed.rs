use core::mem::size_of;

use alloc::vec::Vec;
use pinocchio::Address;

use crate::event_engine::{EventDiscriminator, EventDiscriminators, EventSerialize};

/// Emitted when a bracket's score is refreshed against the oracle.
#[repr(C, packed)]
pub struct ScoreRefreshedEvent {
    pub owner: Address,
    pub score: u16,
}

impl ScoreRefreshedEvent {
    pub const DATA_LEN: usize = size_of::<Self>();

    pub fn new(owner: Address, score: u16) -> Self {
        Self { owner, score }
    }
}

impl EventDiscriminator for ScoreRefreshedEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::ScoreRefreshed as u8;
}

impl EventSerialize for ScoreRefreshedEvent {
    const DATA_LEN: usize = Self::DATA_LEN;

    fn write_inner(&self, writer: &mut Vec<u8>) {
        let score = self.score;
        writer.extend_from_slice(self.owner.as_ref());
        writer.extend_from_slice(&score.to_le_bytes());
    }
}
