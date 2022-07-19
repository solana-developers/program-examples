use borsh::{ BorshSerialize, BorshDeserialize };


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenMetadata {
    pub title: String,
    pub symbol: String,
    pub uri: String,
    pub mint_authority_pda_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MintTokensTo {
    pub amount: u64,
    pub mint_authority_pda_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TransferTokensTo {
    pub amount: u64,
}