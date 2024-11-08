use steel::*;

/// account discriminator
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    User = 0,
}

account!(SteelAccount, User);
/// User
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct User {
    pub name: [u8; 48],
}

impl User {
    pub const SEED_PREFIX: &'static str = "USER";
}
