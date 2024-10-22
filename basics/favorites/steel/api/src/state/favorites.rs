use steel::*;

use super::SteelAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Favorites {
    pub number: u64,

    pub color: [u8; 32],

    pub hobbies: [[u8; 32]; 3],
}

account!(SteelAccount, Favorites);
