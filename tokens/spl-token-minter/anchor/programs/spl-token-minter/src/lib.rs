use anchor_lang::prelude::*;

pub mod instructions;

use instructions::*;


declare_id!("5BStrknWEiQWHmDGdDBaG7j9qegfN2Nq83M3hXajjgXY");


#[program]
pub mod spl_token_minter {
    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>, 
        token_title: String, 
        token_symbol: String, 
        token_uri: String,
    ) -> Result<()> {

        create::create_token(
            ctx, 
            token_title, 
            token_symbol, 
            token_uri,
        )
    }

    pub fn mint_to(
        ctx: Context<MintTo>, 
        quantity: u64,
    ) -> Result<()> {

        mint::mint_to(
            ctx, 
            quantity,
        )
    }
}
