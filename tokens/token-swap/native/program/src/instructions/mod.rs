use borsh::{BorshDeserialize, BorshSerialize};

pub mod create_amm;
pub mod create_pool;
pub mod deposit_liquidity;

pub use create_amm::{process_create_amm, CreateAmmArgs};
pub use create_pool::{process_create_pool, CreatePoolArgs};
pub use deposit_liquidity::{process_deposit_liquidity, DepositLiquidityArgs};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum AmmInstruction {
    CreateAmm(CreateAmmArgs),
    CreatePool(CreatePoolArgs),
    DepositLiquidity(DepositLiquidityArgs),
}
