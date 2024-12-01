pub mod amm;
pub mod pool;

pub use amm::*;
pub use pool::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    Amm = 0,
    Pool = 1,
}
