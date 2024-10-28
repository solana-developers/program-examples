use steel::*;

/// Declare custom error enum 
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum CreateAccountError {
    /// Discriminator for error is set to '0'
    #[error("There was an error while creating your account")]
    AccountCreation = 0,
}

error!(CreateAccountError);
