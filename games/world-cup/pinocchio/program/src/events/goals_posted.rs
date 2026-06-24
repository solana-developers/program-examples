use core::mem::size_of;

use alloc::vec::Vec;

use crate::event_engine::{EventDiscriminator, EventDiscriminators, EventSerialize};

/// Emitted when the admin posts the Round-of-32 goal total.
#[repr(C, packed)]
pub struct GoalsPostedEvent {
    pub total_goals_r32: u16,
}

impl GoalsPostedEvent {
    pub const DATA_LEN: usize = size_of::<Self>();

    pub fn new(total_goals_r32: u16) -> Self {
        Self { total_goals_r32 }
    }
}

impl EventDiscriminator for GoalsPostedEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::GoalsPosted as u8;
}

impl EventSerialize for GoalsPostedEvent {
    const DATA_LEN: usize = Self::DATA_LEN;

    fn write_inner(&self, writer: &mut Vec<u8>) {
        let total = self.total_goals_r32;
        writer.extend_from_slice(&total.to_le_bytes());
    }
}
