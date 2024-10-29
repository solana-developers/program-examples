use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    MintAuthorityPda = 0,
}

account!(SteelAccount, MintAuthorityPda);
/// Mint Authority PDA
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct MintAuthorityPda {
    pub bump: u8,
}

impl MintAuthorityPda {
    pub const SEED_PREFIX: &'static str = "mint_authority";
}
