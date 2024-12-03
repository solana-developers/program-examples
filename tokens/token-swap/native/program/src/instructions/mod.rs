use borsh::{BorshDeserialize, BorshSerialize};

pub mod create_amm;

pub use create_amm::*;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum AmmInstruction {
    CreateAmm(CreateAmmArgs),
}
