use core::mem::size_of;

use alloc::vec::Vec;
use pinocchio::Address;

use crate::event_engine::{EventDiscriminator, EventDiscriminators, EventSerialize};

/// Emitted when an entrant closes their bracket to reclaim its rent.
#[repr(C, packed)]
pub struct BracketClosedEvent {
    pub owner: Address,
}

impl BracketClosedEvent {
    pub const DATA_LEN: usize = size_of::<Self>();

    pub fn new(owner: Address) -> Self {
        Self { owner }
    }
}

impl EventDiscriminator for BracketClosedEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::BracketClosed as u8;
}

impl EventSerialize for BracketClosedEvent {
    const DATA_LEN: usize = Self::DATA_LEN;

    fn write_inner(&self, writer: &mut Vec<u8>) {
        writer.extend_from_slice(self.owner.as_ref());
    }
}
