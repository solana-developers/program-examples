#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;

declare_id!("A5gNtapBvMLD6i7D2t3SSyJeFtBdfb6ibvZu1hoBLzCo");

#[program]
pub mod token_minter {
    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        create::create_token(ctx, token_name, token_symbol, token_uri)
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        mint::mint_token(ctx, amount)
    }
}
