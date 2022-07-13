use borsh::{ BorshSerialize, BorshDeserialize };


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenMetadata {
    pub title: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MintTokenTo {
    pub amount: u64,
}