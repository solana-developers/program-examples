use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum AccountInstruction {
    InitializeAccount = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InitializeAccount {
    pub name: [u8; 64],
    pub house_number: u8, 
    pub city: [u8; 64],
    pub street: [u8; 64],

}

instruction!(AccountInstruction, InitializeAccount);


