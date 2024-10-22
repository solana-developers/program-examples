use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    SetFavorites = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetFavorites {
    pub number: [u8; 8],

    // pub color: String,
    pub color: [u8; 64],

    // pub hobbies: Vec<String>,
    pub hobbies: [[u8; 64]; 5],
}

instruction!(SteelInstruction, SetFavorites);
