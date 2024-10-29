use super::SteelAccount;
use steel::*;

account!(SteelAccount, RentVault);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct RentVault {}

impl RentVault {
    pub const SEED_PREFIX: &'static str = "rent_vault";
}
