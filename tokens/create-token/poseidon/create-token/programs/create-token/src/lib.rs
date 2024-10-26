// Workaround implementation for initializing the token mint
// Note: Poseidon's transpiler does not support initializeMint yet,
// so this is done manually using Anchor's InitializeMint

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

declare_id!("FThBfjqE8JBZYX8SdiJtDZwwGuVaQVMFfcR9JEsxS2A");

#[program]
pub mod create_token {
    use super::*;

    pub fn create_token_mint(_ctx: Context<CreateTokenMintContext>, _decimals: u8) -> Result<()> {
        // Workaround implementation for initializing the token mint
        // Note: Poseidon's transpiler does not support initializeMint yet,
        // so this is done manually using Anchor's InitializeMint.
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_decimals: u8)]
pub struct CreateTokenMintContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // Create new mint account
    #[account(
        init,
        payer = payer,
        mint::decimals = _decimals,
        mint::authority = payer.key(),
    )]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
