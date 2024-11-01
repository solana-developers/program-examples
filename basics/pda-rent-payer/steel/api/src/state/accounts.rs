use super::PdaRentPayerAccountDiscriminator;
use steel::*;

/// This empty struct represents the payer vault account
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct RentVault {}

/// This empty struct represents the account
/// that the vault will pay for
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct NewAccount {}

account!(PdaRentPayerAccountDiscriminator, RentVault);
account!(PdaRentPayerAccountDiscriminator, NewAccount);
