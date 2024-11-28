use steel::*;

// First, define an enum for the account discriminator
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountType {
    MintAuthorityPda = 0,
}

// Now define the actual account state
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct MintAuthorityPda {
    pub bump: u8,
}

impl MintAuthorityPda {
    pub const SEED_PREFIX: &'static str = "mint_authority";
    pub const SIZE: usize = 1; // 1 byte for bump

}

// Use the account! macro to implement necessary traits
account!(AccountType, MintAuthorityPda);

// Helper function to get the total size of the account, including discriminator
pub fn get_mint_authority_pda_size() -> usize {
    8 + MintAuthorityPda::SIZE // 8 bytes for discriminator + size of state
}

pub fn mint_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[MintAuthorityPda::SEED_PREFIX.as_bytes()], &crate::id())
}