use steel::*;
use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum FavoritesInstruction {
    SetFavoritesArgs = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetFavoritesArgs {
    pub number: u64,
    pub color_len: u32,
    pub color: [u8; STRING_MAX_SIZE],
    pub hobbies_count: u32,
    pub hobbies_len: [u32; MAX_HOBBIES],
    pub hobbies: [[u8; STRING_MAX_SIZE]; MAX_HOBBIES],
}


instruction!(FavoritesInstruction, SetFavoritesArgs);

