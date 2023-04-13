use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("DU92SqPvfiDVicA4ah3E9wP9Ci7ASog1VmJhZrYU3Hyk");

#[program]
pub mod nft_minter {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        init::init(ctx)
    }

    pub fn create_token(
        ctx: Context<CreateToken>,
        nft_title: String,
        nft_symbol: String,
        nft_uri: String,
    ) -> Result<()> {
        create::create_token(ctx, nft_title, nft_symbol, nft_uri)
    }

    pub fn mint_to(ctx: Context<MintTo>) -> Result<()> {
        mint::mint_to(ctx)
    }
}
