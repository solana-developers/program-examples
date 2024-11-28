use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum PdaRentPayerError {
    #[error("Rent vault account already initialized")]
    RentVaultInitialized = 0,
}

error!(PdaRentPayerError);
