use core::mem::size_of;

use alloc::vec::Vec;

use crate::event_engine::{EventDiscriminator, EventDiscriminators, EventSerialize};

/// Emitted when the tournament transitions from registration to locked.
#[repr(C, packed)]
pub struct TournamentLockedEvent {
    pub lock_ts: i64,
}

impl TournamentLockedEvent {
    pub const DATA_LEN: usize = size_of::<Self>();

    pub fn new(lock_ts: i64) -> Self {
        Self { lock_ts }
    }
}

impl EventDiscriminator for TournamentLockedEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::TournamentLocked as u8;
}

impl EventSerialize for TournamentLockedEvent {
    const DATA_LEN: usize = Self::DATA_LEN;

    fn write_inner(&self, writer: &mut Vec<u8>) {
        let lock_ts = self.lock_ts;
        writer.extend_from_slice(&lock_ts.to_le_bytes());
    }
}
