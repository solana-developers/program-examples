use anchor_lang::prelude::*;

pub mod instructions;
use instructions::*;

declare_id!("3of89Z9jwek9zrFgpCWc9jZvQvitpVMxpZNsrAD2vQUD");

#[program]
pub mod spl_token_minter {
    use super::*;

    pub fn create_token(
        context: Context<CreateTokenAccountConstraints>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        create::handle_create_token(context, token_name, token_symbol, token_uri)
    }

    pub fn mint_token(context: Context<MintTokenAccountConstraints>, amount: u64) -> Result<()> {
        mint::handle_mint_token(context, amount)
    }
}
