use steel::*;
use crate::state::AddressInfoData;

/// Instruction types for the address info program
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum AddressInfoInstruction {
    CreateAddressInfo = 0,
}

/// Instruction data for creating address info
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateAddressInfo {
    pub data: AddressInfoData,
}

// Link instruction type with its data structure
instruction!(AddressInfoInstruction, CreateAddressInfo);