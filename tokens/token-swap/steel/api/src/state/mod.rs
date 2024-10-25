mod amm;

pub use amm::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum TokenSwapAccount {
    Amm = 0,
}
