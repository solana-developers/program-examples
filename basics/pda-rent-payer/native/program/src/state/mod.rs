use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct RentVault {}

impl RentVault {
    pub const SEED_PREFIX: &'static str = "rent_vault";
}
