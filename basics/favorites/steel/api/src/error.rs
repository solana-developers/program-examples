use steel::*;

#[repr(u32)]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
pub enum FavoritesError {
    #[error("String too long")]
    StringTooLong = 0,
    #[error("Too many hobbies")]
    TooManyHobbies = 1,
}

error!(FavoritesError);