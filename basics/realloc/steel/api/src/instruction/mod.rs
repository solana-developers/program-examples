pub mod create;
pub use create::*;

pub mod extend;
pub use extend::*;

pub mod zero_init;
pub use zero_init::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]

pub enum SteelInstruction {
    CreateAddressInfo = 0,
    ExtendAddressInfo = 1,
    ZeroInit = 2,
}
