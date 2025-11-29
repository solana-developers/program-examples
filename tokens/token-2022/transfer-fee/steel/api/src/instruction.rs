use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    Initialize = 0,
    Transfer = 1,
    Harvest = 2,
    Withdraw = 3,
    UpdateFee = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {
    pub maximum_fee: [u8; 8], //NOTE: take into consideration the token decimals, you can also scale the input to match the decimals
    pub transfer_fee_basis_points: [u8; 2],
    pub decimals: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Transfer {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Harvest {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Withdraw {
    pub destination: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UpdateFee {
    pub maximum_fee: [u8; 8], //NOTE: take into consideration the token decimals, you can also scale the input to match the decimals.
    pub transfer_fee_basis_points: [u8; 2],
}

instruction!(SteelInstruction, Initialize);
instruction!(SteelInstruction, Transfer);
instruction!(SteelInstruction, Harvest);
instruction!(SteelInstruction, Withdraw);
instruction!(SteelInstruction, UpdateFee);
