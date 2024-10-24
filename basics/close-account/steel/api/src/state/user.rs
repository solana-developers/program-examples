use steel::*;

use crate::error::{CloseAccountError, CloseAccountResult};

/// An enum which is used to derive a discriminator for the user account.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum UserAccount {
    /// The user is represented by a discriminator of `0`
    User = 0,
}

/// The user Account structure which stores a
/// `name` as bytes with max array length of u64 due to the
/// requirement for memory alignment since 64 is a factor of 8.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct User {
    /// The name string stored as bytes.
    /// The `&str` is converted into bytes and copied upto
    /// the length of the bytes, if the bytes are not 64, it
    /// pads with zeroes upto 64, if it is more than 64 an error
    /// is returned.
    pub name: [u8; 64],
}

impl User {
    /// Seed for the [User] used to in PDA generation
    pub const SEED_PREFIX: &'static str = "USER";

    /// Create a new user, convert the name into bytes
    /// and add those bytes to a 64 byte array
    pub fn new(name: &str) -> CloseAccountResult<Self> {
        let name_bytes = name.as_bytes();

        Self::check_length(name_bytes)?;

        let mut name = [0u8; 64];
        name[0..name_bytes.len()].copy_from_slice(name_bytes);

        Ok(Self { name })
    }

    /// Converts the byte array into a UTF-8 [str]
    /// using the `trim_end_matches("\0")` of [str] method
    /// to remove padded zeroes if any. Padded zeroes are
    /// represented by `\0`
    pub fn to_string(&self) -> CloseAccountResult<String> {
        let value =
            core::str::from_utf8(&self.name).map_err(|_| CloseAccountError::OnlyUtf8IsSupported)?;

        Ok(value.trim_end_matches("\0").to_string())
    }

    fn check_length(bytes: &[u8]) -> CloseAccountResult<()> {
        if bytes.len() > 64 {
            return Err(CloseAccountError::MaxNameLengthExceeded);
        }

        Ok(())
    }

    /// Generate a PDA from the [Self::SEED_PREFIX] constant
    /// and the payer public key. This returns a tuple struct
    /// ([Pubkey], [u8])
    pub fn pda(payer: Pubkey) -> (Pubkey, u8) {
        Pubkey::try_find_program_address(
            &[Self::SEED_PREFIX.as_bytes(), payer.as_ref()],
            &crate::id(),
        )
        .unwrap()
    }
}

account!(UserAccount, User);
