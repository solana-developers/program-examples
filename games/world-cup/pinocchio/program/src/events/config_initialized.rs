use core::mem::size_of;

use alloc::vec::Vec;
use pinocchio::Address;

use crate::event_engine::{EventDiscriminator, EventDiscriminators, EventSerialize};

/// Emitted when the tournament config is initialized.
#[repr(C, packed)]
pub struct ConfigInitializedEvent {
    pub admin: Address,
    pub lock_ts: i64,
    pub entry_fee: u64,
}

impl ConfigInitializedEvent {
    pub const DATA_LEN: usize = size_of::<Self>();

    pub fn new(admin: Address, lock_ts: i64, entry_fee: u64) -> Self {
        Self { admin, lock_ts, entry_fee }
    }
}

impl EventDiscriminator for ConfigInitializedEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::ConfigInitialized as u8;
}

impl EventSerialize for ConfigInitializedEvent {
    const DATA_LEN: usize = Self::DATA_LEN;

    fn write_inner(&self, writer: &mut Vec<u8>) {
        let lock_ts = self.lock_ts;
        let entry_fee = self.entry_fee;
        writer.extend_from_slice(self.admin.as_ref());
        writer.extend_from_slice(&lock_ts.to_le_bytes());
        writer.extend_from_slice(&entry_fee.to_le_bytes());
    }
}
