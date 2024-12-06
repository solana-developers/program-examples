use borsh::{BorshDeserialize, BorshSerialize};

pub mod create_amm;
pub mod create_pool;
pub mod deposit_liquidity;
pub mod swap_exact_tokens_for_tokens;

pub use create_amm::{process_create_amm, CreateAmmArgs};
pub use create_pool::{process_create_pool, CreatePoolArgs};
pub use deposit_liquidity::{process_deposit_liquidity, DepositLiquidityArgs};
pub use swap_exact_tokens_for_tokens::{
    process_swap_exact_tokens_for_tokens, SwapExactTokensForTokensArgs,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum AmmInstruction {
    CreateAmm(CreateAmmArgs),
    CreatePool(CreatePoolArgs),
    DepositLiquidity(DepositLiquidityArgs),
    SwapExactTokensForToken(SwapExactTokensForTokensArgs),
}
