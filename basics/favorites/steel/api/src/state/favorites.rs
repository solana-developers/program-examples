use steel::*;

use super::SteelAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Favorites {
    pub number: u64,

    // pub color: String,
    pub color: [u8; 64],

    // pub hobbies: Vec<String>,
    pub hobbies: [[u8; 64]; 5],
}

account!(SteelAccount, Favorites);
