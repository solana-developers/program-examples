use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid price provided")]
    InvalidPrice,

    #[msg("Invalid quantity provided")]
    InvalidQuantity,

    #[msg("Order not found")]
    OrderNotFound,

    #[msg("Market is currently paused")]
    MarketPaused,

    #[msg("Unauthorized action")]
    Unauthorized,

    #[msg("Order book is full")]
    OrderBookFull,

    #[msg("User account has too many open orders")]
    TooManyOpenOrders,

    #[msg("Price does not align with tick size")]
    InvalidTickSize,

    #[msg("Quantity is below minimum order size")]
    BelowMinOrderSize,

    #[msg("Order is not cancellable in current status")]
    OrderNotCancellable,

    #[msg("Numerical overflow occurred")]
    NumericalOverflow,

    #[msg("Fee basis points out of range")]
    InvalidFeeBasisPoints,

    #[msg("Fee vault does not match the market's fee vault")]
    InvalidFeeVault,

    #[msg("Maker account provided does not correspond to a resting order on the book")]
    MakerAccountMismatch,

    #[msg("Not enough maker accounts supplied to cross the incoming order")]
    MissingMakerAccounts,

    #[msg("Maker order and maker user account owner mismatch")]
    MakerOwnerMismatch,

    #[msg("Only the market authority can withdraw fees")]
    NotMarketAuthority,
}
