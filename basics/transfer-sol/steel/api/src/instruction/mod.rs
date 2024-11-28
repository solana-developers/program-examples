pub mod with_cpi;
pub mod with_program;

pub use with_cpi::*;
pub use with_program::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TransferInstruction {
    TransferSolWithCpi = 0,
    TransferSolWithProgram = 1,
}
