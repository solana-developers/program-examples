use anchor_lang::prelude::*;

pub mod instructions;

use instructions::*;


declare_id!("A6itasS5iqANkC9yrzP1HJPBnJxj9tC8G5TmJzQGogGG");


#[program]
pub mod nft_minter {
    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>, 
        nft_title: String, 
        nft_symbol: String, 
        nft_uri: String,
    ) -> Result<()> {

        create::create_token(
            ctx, 
            nft_title, 
            nft_symbol, 
            nft_uri,
        )
    }

    pub fn mint_to(
        ctx: Context<MintTo>, 
    ) -> Result<()> {

        mint::mint_to(
            ctx, 
        )
    }
}
