pub mod make_offer;
pub mod take_offer;

pub use make_offer::*;
pub use take_offer::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum EscrowInstruction {
    MakeOffer = 0,
    TakeOffer = 1,
}
