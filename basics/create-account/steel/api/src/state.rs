use steel::*;

/// This enum is used to get a discriminator
/// for the new account.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum CreateAccountDiscriminator {
    NewAccount = 0,
}

/// This empty struct represents the system account
/// It contains no data and is used to create a new account
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct NewAccount {}

account!(CreateAccountDiscriminator, NewAccount);
