use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;

#[derive(ShankAccount, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Counter {
    pub count: u64,
}
