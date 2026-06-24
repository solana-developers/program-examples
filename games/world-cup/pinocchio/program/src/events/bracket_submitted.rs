use core::mem::size_of;

use alloc::vec::Vec;
use pinocchio::Address;

use crate::event_engine::{EventDiscriminator, EventDiscriminators, EventSerialize};

/// Emitted when an entrant submits a bracket. Carries the full submission (picks +
/// tiebreaker) so it survives in the ledger after the bracket PDA is closed.
#[repr(C, packed)]
pub struct BracketSubmittedEvent {
    pub owner: Address,
    pub entrant_count: u32,
    pub tiebreaker_guess: u16,
    pub picks: [u8; 32],
}

impl BracketSubmittedEvent {
    pub const DATA_LEN: usize = size_of::<Self>();

    pub fn new(owner: Address, entrant_count: u32, tiebreaker_guess: u16, picks: [u8; 32]) -> Self {
        Self { owner, entrant_count, tiebreaker_guess, picks }
    }
}

impl EventDiscriminator for BracketSubmittedEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::BracketSubmitted as u8;
}

impl EventSerialize for BracketSubmittedEvent {
    const DATA_LEN: usize = Self::DATA_LEN;

    fn write_inner(&self, writer: &mut Vec<u8>) {
        let entrant_count = self.entrant_count;
        let tiebreaker_guess = self.tiebreaker_guess;
        writer.extend_from_slice(self.owner.as_ref());
        writer.extend_from_slice(&entrant_count.to_le_bytes());
        writer.extend_from_slice(&tiebreaker_guess.to_le_bytes());
        writer.extend_from_slice(&self.picks);
    }
}
