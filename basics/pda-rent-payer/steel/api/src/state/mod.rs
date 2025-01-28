mod accounts;

use crate::consts::*;
pub use accounts::*;
use steel::*;

/// This enum represents the discriminator for the
/// accounts this program can interact with
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum PdaRentPayerAccountDiscriminator {
    RentVault = 0,
    NewAccount = 1,
}

/// Fetch PDA of the rent_vault account.
pub fn rent_vault_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[RENT_VAULT], &crate::id())
}
