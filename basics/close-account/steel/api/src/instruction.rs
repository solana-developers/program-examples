use steel::*;

/// Used in generating the discriminats for instructions
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum MyInstruction {
    /// Create account discriminant represented by `0`
    CreateAccount = 0,
    /// Close account discriminant represented by `1`
    CloseAccount = 1,
}

/// Create account struct with the name
/// as an array of 64 bytes
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateAccount(pub [u8; 64]);

/// UsedClose Account
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CloseAccount;

instruction!(MyInstruction, CreateAccount);
instruction!(MyInstruction, CloseAccount);
