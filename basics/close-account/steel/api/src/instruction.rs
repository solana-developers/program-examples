use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum CloseAccountInstruction {
    CreateUser = 0,
    CloseUser = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateUser {
    pub name: [u8; 64],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CloseUser {}

instruction!(CloseAccountInstruction, CreateUser);
instruction!(CloseAccountInstruction, CloseUser);
