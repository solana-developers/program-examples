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
}
