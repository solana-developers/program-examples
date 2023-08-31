use {
    borsh::{
        BorshDeserialize,
        BorshSerialize
    },
    shank::ShankAccount,
    solana_program::pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankAccount)]
#[seeds(
    "car",
    program_id,
    make("The car's make", String),
    model("The car's model", String),
)]
pub struct Car {
    pub year: u16,
    pub make: String,
    pub model: String,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum RentalOrderStatus {
    Created,
    PickedUp,
    Returned,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankAccount)]
#[seeds(
    "rental_order",
    program_id,
    car_public_key("The car's public key", Pubkey),
    payer_public_key("The payer's public key", Pubkey),
)]
pub struct RentalOrder {
    pub car: Pubkey,
    pub name: String,
    pub pick_up_date: String,
    pub return_date: String,
    pub price: u64,
    pub status: RentalOrderStatus,
}

impl RentalOrder {
    pub const SEED_PREFIX: &'static str = "rental_order";
}
