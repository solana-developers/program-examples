use anchor_lang::prelude::*;

pub mod instructions;

use instructions::*;


declare_id!("86NFhnnuV8SNEuHdqNtjCFeGCknpjDXEbV7b3zBefJ11");


#[program]
pub mod mint_token_to {
    use super::*;

    pub fn create_token_mint(
        ctx: Context<CreateTokenMint>, 
        metadata_title: String, 
        metadata_symbol: String, 
        metadata_uri: String,
        mint_authority_pda_bump: u8,
    ) -> Result<()> {

        create_token_mint::create_token_mint(
            ctx, 
            metadata_title, 
            metadata_symbol, 
            metadata_uri,
            mint_authority_pda_bump,
        )
    }

    pub fn mint_to_wallet(
        ctx: Context<MintToWallet>, 
        amount: u64,
        mint_authority_pda_bump: u8,
    ) -> Result<()> {

        mint_to_wallet::mint_to_wallet(
            ctx, 
            amount,
            mint_authority_pda_bump,
        )
    }
}
