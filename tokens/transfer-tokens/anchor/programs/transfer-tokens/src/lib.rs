#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod instructions;

use instructions::*;

declare_id!("7p8osL5uUKUFM8sUxRYfmFVGN264PxeBSDEdNv36Khe3");

#[program]
pub mod transfer_tokens {
    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>,
        token_title: String,
        token_symbol: String,
        token_uri: String,
        decimals: u8,
    ) -> Result<()> {
        create::create_token(ctx, token_title, token_symbol, token_uri, decimals)
    }

    pub fn mint_spl(ctx: Context<MintSpl>, quantity: u64) -> Result<()> {
        mint_spl::mint_spl(ctx, quantity)
    }

    pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {
        mint_nft::mint_nft(ctx)
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, quantity: u64) -> Result<()> {
        transfer::transfer_tokens(ctx, quantity)
    }
}
