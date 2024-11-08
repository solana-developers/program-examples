pub mod init_rent_vault;
pub use init_rent_vault::*;

pub mod create_new_account;
pub use create_new_account::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]

pub enum SteelInstruction {
    InitRentVault = 0,
    CreateNewAccount = 1,
}
