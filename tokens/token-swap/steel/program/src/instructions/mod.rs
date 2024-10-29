pub mod create_amm;
pub mod create_pool;
pub mod deposit_liquidity;
pub mod swap_exact_tokens_for_tokens;
pub mod withdraw_liquidity;

pub use create_amm::*;
pub use create_pool::*;
pub use deposit_liquidity::*;
pub use swap_exact_tokens_for_tokens::*;
pub use withdraw_liquidity::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    CreateAmm = 0,
    CreatePool = 1,
    DepositLiquidity = 2,
    SwapExactTokensForTokens = 3,
    WithdrawLiquidity = 4,
}
