use steel::*;

use super::FavoritesAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Favorites {
    pub number: u64,
    pub color_len: u32,
    pub color: [u8; STRING_MAX_SIZE],
    pub hobbies_count: u32,
    pub hobbies_len: [u32; MAX_HOBBIES],
    pub hobbies: [[u8; STRING_MAX_SIZE]; MAX_HOBBIES],
}

account!(FavoritesAccount, Favorites);
