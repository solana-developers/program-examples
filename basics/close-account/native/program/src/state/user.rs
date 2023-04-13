use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    pub name: String,
}

impl User {
    pub const SEED_PREFIX: &'static str = "USER";
}
