use core::mem::size_of;

use alloc::vec::Vec;

use crate::event_engine::{EventDiscriminator, EventDiscriminators, EventSerialize};

/// Emitted when the admin posts a game result.
#[repr(C, packed)]
pub struct ResultPostedEvent {
    pub game: u8,
    pub winner: u8,
    pub decided_mask: u32,
}

impl ResultPostedEvent {
    pub const DATA_LEN: usize = size_of::<Self>();

    pub fn new(game: u8, winner: u8, decided_mask: u32) -> Self {
        Self { game, winner, decided_mask }
    }
}

impl EventDiscriminator for ResultPostedEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::ResultPosted as u8;
}

impl EventSerialize for ResultPostedEvent {
    const DATA_LEN: usize = Self::DATA_LEN;

    fn write_inner(&self, writer: &mut Vec<u8>) {
        let decided_mask = self.decided_mask;
        writer.push(self.game);
        writer.push(self.winner);
        writer.extend_from_slice(&decided_mask.to_le_bytes());
    }
}
