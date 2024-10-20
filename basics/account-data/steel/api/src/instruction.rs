use steel::*;

use crate::state::AddressInfoData;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum AddressInfoInstruction {
    CreateAddressInfo = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateAddressInfo {
    pub data: AddressInfoData,
}

instruction!(AddressInfoInstruction, CreateAddressInfo);
