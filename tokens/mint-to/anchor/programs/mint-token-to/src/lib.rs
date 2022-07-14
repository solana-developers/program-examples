use anchor_lang::prelude::*;

pub mod instructions;

use instructions::*;


declare_id!("DDvuxbzPh3aSb67Kc7kvxb7SsU6bNmPBfqsqvQFHcqWu");


#[program]
pub mod mint_token_to {
    use super::*;

    pub fn create_token_mint(
        ctx: Context<CreateTokenMint>, 
        metadata_title: String, 
        metadata_symbol: String, 
        metadata_uri: String,
    ) -> Result<()> {

        create_token_mint::create_token_mint(
            ctx, 
            metadata_title, 
            metadata_symbol, 
            metadata_uri
        )
    }

    pub fn mint_to_wallet(
        ctx: Context<MintToWallet>, 
        amount: u64,
    ) -> Result<()> {

        mint_to_wallet::mint_to_wallet(ctx, amount)
    }
}
