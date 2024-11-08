pub mod rent_vault;
pub use rent_vault::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    RentVault = 0,
}
