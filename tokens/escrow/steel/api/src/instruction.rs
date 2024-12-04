use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum EscrowInstruction {
    MakeOffer = 0,
    TakeOffer = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MakeOffer {
    pub token_a_offered_amount: [u8; 8],
    pub id: [u8; 8],
    pub token_b_wanted_amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TakeOffer {}

instruction!(EscrowInstruction, MakeOffer);
instruction!(EscrowInstruction, TakeOffer);
