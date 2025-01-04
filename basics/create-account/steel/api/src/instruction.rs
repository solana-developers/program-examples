use steel::*;

/// Declare the Instructions enum for create account
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum CreateAccountInstruction {
    /// Initialize account discriminator set to '0'
    InitializeAccount = 0,
}

/// Empty initialize account struct since
/// no data input is needed
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InitializeAccount {}

// Link Instructions enum to variant
instruction!(CreateAccountInstruction, InitializeAccount);
