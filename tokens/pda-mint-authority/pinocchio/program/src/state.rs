use pinocchio::error::ProgramError;

/// Persistent record stored in the mint-authority PDA.
///
/// The PDA is derived from `[b"mint_authority"]` and acts as the mint and
/// freeze authority for every token this program creates. Persisting the
/// canonical bump lets later instructions rebuild the signer seeds without
/// re-deriving the address on-chain.
pub struct MintAuthorityPda {
    /// Canonical bump for the mint-authority PDA.
    pub bump: u8,
}

impl MintAuthorityPda {
    /// Seed for the mint-authority PDA: `[SEED_PREFIX]`.
    pub const SEED_PREFIX: &'static [u8] = b"mint_authority";

    /// Bytes allocated for the account. Mirrors the `native` example (8 + 8);
    /// only the first byte (the bump) is meaningful.
    pub const ACCOUNT_SPACE: usize = 16;

    /// Writes the bump into the first byte of `dst`.
    pub fn serialize(&self, dst: &mut [u8]) -> Result<(), ProgramError> {
        *dst.first_mut().ok_or(ProgramError::AccountDataTooSmall)? = self.bump;
        Ok(())
    }

    /// Reads the bump from the first byte of `src`.
    pub fn deserialize(src: &[u8]) -> Result<Self, ProgramError> {
        let bump = *src.first().ok_or(ProgramError::InvalidAccountData)?;
        Ok(Self { bump })
    }
}
