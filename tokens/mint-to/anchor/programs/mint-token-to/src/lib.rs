use anchor_lang::prelude::*;

pub mod instructions;

use instructions::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


#[program]
pub mod mint_token_to {
    use super::*;

    pub fn create_token_mint(
        ctx: Context<CreateTokenMint>, 
        metadata_title: String, 
        metadata_symbol: String, 
        metadata_uri: String,
    ) -> Result<()> {

        instructions::create_token_mint::create_token_mint(
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

        instructions::mint_to_wallet::mint_to_wallet(ctx, amount)
    }
}
