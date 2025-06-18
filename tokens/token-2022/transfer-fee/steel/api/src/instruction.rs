use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    Initialize = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {
    pub maximum_fee: [u8; 8], //NOTE: take into consideration the token decimals or maybe I should just scale using the input
    pub transfer_fee_basis_points: [u8; 2],
    pub decimals: u8,
}

instruction!(SteelInstruction, Initialize);
