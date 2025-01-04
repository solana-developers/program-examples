use steel::*;

use crate::state::PageVisits;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum ProgramDerivedAddressesInstruction {
    Create = 0,
    Increment = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Create {
    pub page_visits: PageVisits,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Increment {}

instruction!(ProgramDerivedAddressesInstruction, Create);
instruction!(ProgramDerivedAddressesInstruction, Increment);
