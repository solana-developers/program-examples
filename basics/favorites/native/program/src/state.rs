use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Favorites {
    pub number: u64,
    pub color: String,
    pub hobbies: Vec<String>
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GetFavorites {
}

