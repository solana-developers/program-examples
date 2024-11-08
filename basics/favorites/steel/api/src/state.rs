use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    Favorites = 0,
}

account!(SteelAccount, Favorites);
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Favorites {
    pub number: u64,
    pub color: [u8; 48],
    pub hobbies: [[u8; 48]; 5],
}
