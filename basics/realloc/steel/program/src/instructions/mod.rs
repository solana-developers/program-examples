pub mod create;
pub use create::*;

pub mod extend;
pub use extend::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]

pub enum SteelInstruction {
    CreateAddressInfo = 0,
    ExtendAddressInfo = 1,
}
