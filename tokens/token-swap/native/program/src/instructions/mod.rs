use borsh::{BorshDeserialize, BorshSerialize};

pub mod create_amm;
pub mod create_pool;

pub use create_amm::{process_create_amm, CreateAmmArgs};
pub use create_pool::{process_create_pool, CreatePoolArgs};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum AmmInstruction {
    CreateAmm(CreateAmmArgs),
    CreatePool(CreatePoolArgs),
}
