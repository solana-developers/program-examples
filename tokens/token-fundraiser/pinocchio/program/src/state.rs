use pinocchio::error::ProgramError;

/// Persistent state for a fundraiser campaign, stored in the fundraiser PDA.
///
/// The fundraiser PDA is derived from `[b"fundraiser", maker]` and is the
/// authority of the vault token account that custodies contributed tokens until
/// the campaign succeeds (funds go to the maker) or fails (funds are refunded).
///
/// Serialized byte layout (little-endian), matching the field order below so a
/// Borsh client can deserialize it directly:
/// `[maker: 32][mint_to_raise: 32][vault: 32][amount_to_raise: u64]
///  [current_amount: u64][time_started: i64][duration: u16][bump: u8]`
pub struct Fundraiser {
    /// The wallet that created the fundraiser and receives the funds on success.
    pub maker: [u8; 32],
    /// Mint of the token being raised.
    pub mint_to_raise: [u8; 32],
    /// The vault (the fundraiser PDA's associated token account) that holds the
    /// raised funds. Recorded at initialization — where the associated token
    /// program guarantees it is the canonical ATA — so later instructions can
    /// reject any other token account passed in its place.
    pub vault: [u8; 32],
    /// Target amount (in base units) the campaign wants to raise.
    pub amount_to_raise: u64,
    /// Amount contributed so far.
    pub current_amount: u64,
    /// Unix timestamp at which the campaign started.
    pub time_started: i64,
    /// Campaign duration, in days.
    pub duration: u16,
    /// Canonical bump for the fundraiser PDA.
    pub bump: u8,
}

impl Fundraiser {
    /// Seed prefix for the fundraiser PDA: `[SEED_PREFIX, maker]`.
    pub const SEED_PREFIX: &'static [u8] = b"fundraiser";

    /// Serialized size of a `Fundraiser` in bytes.
    pub const LEN: usize = 32 + 32 + 32 + 8 + 8 + 8 + 2 + 1;

    /// Writes the fundraiser into `dst` using the layout documented above.
    pub fn serialize(&self, dst: &mut [u8]) -> Result<(), ProgramError> {
        let dst = dst
            .get_mut(..Self::LEN)
            .ok_or(ProgramError::AccountDataTooSmall)?;
        dst[0..32].copy_from_slice(&self.maker);
        dst[32..64].copy_from_slice(&self.mint_to_raise);
        dst[64..96].copy_from_slice(&self.vault);
        dst[96..104].copy_from_slice(&self.amount_to_raise.to_le_bytes());
        dst[104..112].copy_from_slice(&self.current_amount.to_le_bytes());
        dst[112..120].copy_from_slice(&self.time_started.to_le_bytes());
        dst[120..122].copy_from_slice(&self.duration.to_le_bytes());
        dst[122] = self.bump;
        Ok(())
    }

    /// Reads a fundraiser from `src`, which must be at least [`Fundraiser::LEN`] bytes.
    pub fn deserialize(src: &[u8]) -> Result<Self, ProgramError> {
        let src: &[u8; Self::LEN] = src
            .get(..Self::LEN)
            .and_then(|s| s.try_into().ok())
            .ok_or(ProgramError::InvalidAccountData)?;
        Ok(Self {
            maker: src[0..32].try_into().unwrap(),
            mint_to_raise: src[32..64].try_into().unwrap(),
            vault: src[64..96].try_into().unwrap(),
            amount_to_raise: u64::from_le_bytes(src[96..104].try_into().unwrap()),
            current_amount: u64::from_le_bytes(src[104..112].try_into().unwrap()),
            time_started: i64::from_le_bytes(src[112..120].try_into().unwrap()),
            duration: u16::from_le_bytes(src[120..122].try_into().unwrap()),
            bump: src[122],
        })
    }
}

/// Per-contributor record, stored in the contributor PDA derived from
/// `[b"contributor", fundraiser, contributor]`. Tracks how much the contributor
/// has put in so it can be refunded if the campaign fails.
///
/// Serialized byte layout (little-endian): `[amount: u64]`.
pub struct Contributor {
    /// Total amount this contributor has supplied.
    pub amount: u64,
}

impl Contributor {
    /// Seed prefix for the contributor PDA: `[SEED_PREFIX, fundraiser, contributor]`.
    pub const SEED_PREFIX: &'static [u8] = b"contributor";

    /// Serialized size of a `Contributor` in bytes.
    pub const LEN: usize = 8;

    /// Writes the contributor into `dst` using the layout documented above.
    pub fn serialize(&self, dst: &mut [u8]) -> Result<(), ProgramError> {
        let dst = dst
            .get_mut(..Self::LEN)
            .ok_or(ProgramError::AccountDataTooSmall)?;
        dst[0..8].copy_from_slice(&self.amount.to_le_bytes());
        Ok(())
    }

    /// Reads a contributor from `src`, which must be at least [`Contributor::LEN`] bytes.
    pub fn deserialize(src: &[u8]) -> Result<Self, ProgramError> {
        let src: &[u8; Self::LEN] = src
            .get(..Self::LEN)
            .and_then(|s| s.try_into().ok())
            .ok_or(ProgramError::InvalidAccountData)?;
        Ok(Self {
            amount: u64::from_le_bytes(src[0..8].try_into().unwrap()),
        })
    }
}
