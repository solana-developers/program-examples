pub mod increment;
pub use increment::*;

pub mod create;
pub use create::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]

pub enum SteelInstruction {
    CreatePageVisits = 0,
    IncrementPageVisits = 1,
}
