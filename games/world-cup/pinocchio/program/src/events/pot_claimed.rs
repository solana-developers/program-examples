use core::mem::size_of;

use alloc::vec::Vec;
use pinocchio::Address;

use crate::event_engine::{EventDiscriminator, EventDiscriminators, EventSerialize};

/// Emitted when the unique winner claims the pot.
#[repr(C, packed)]
pub struct PotClaimedEvent {
    pub winner: Address,
    pub amount: u64,
}

impl PotClaimedEvent {
    pub const DATA_LEN: usize = size_of::<Self>();

    pub fn new(winner: Address, amount: u64) -> Self {
        Self { winner, amount }
    }
}

impl EventDiscriminator for PotClaimedEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::PotClaimed as u8;
}

impl EventSerialize for PotClaimedEvent {
    const DATA_LEN: usize = Self::DATA_LEN;

    fn write_inner(&self, writer: &mut Vec<u8>) {
        let amount = self.amount;
        writer.extend_from_slice(self.winner.as_ref());
        writer.extend_from_slice(&amount.to_le_bytes());
    }
}
