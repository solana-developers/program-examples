#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

/// SPL token minter with Metaplex metadata.
///
/// Two instructions:
/// - `create_token` — creates a mint and associated Metaplex metadata account
/// - `mint_token` — mints tokens to a recipient's associated token account
#[program]
mod quasar_spl_token_minter {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn create_token(
        ctx: Ctx<CreateToken>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<(), ProgramError> {
        instructions::handle_create_token(&mut ctx.accounts, &token_name, &token_symbol, &token_uri)
    }

    #[instruction(discriminator = 1)]
    pub fn mint_token(ctx: Ctx<MintToken>, amount: u64) -> Result<(), ProgramError> {
        instructions::handle_mint_token(&mut ctx.accounts, amount)
    }
}
