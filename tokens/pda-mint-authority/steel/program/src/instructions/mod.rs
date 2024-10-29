pub mod create;
pub mod init;
pub mod mint;

pub use create::*;
pub use init::*;
pub use mint::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    Init = 0,
    CreateToken = 1,
    MintTo = 2,
}
