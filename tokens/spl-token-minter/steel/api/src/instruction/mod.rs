pub mod create;
pub use create::*;

pub mod mint;
pub use mint::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    CreateToken = 0,
    MintTo = 1,
}
