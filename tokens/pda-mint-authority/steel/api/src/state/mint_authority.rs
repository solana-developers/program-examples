use steel::*;

use super::SteelAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct MintAuthorityPda {
    bump: u8,
}

account!(SteelAccount, MintAuthorityPda);
