use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, mint_to, TokenAccount, Mint, MintTo},
};
declare_id!("CSi4VcU9g99HKSodPV3MJvweoEAuaqWqgEC3jvdHieDG");
#[program]
pub mod spl_token_minter {
    use super::*;
    pub fn create_token_mint(
        ctx: Context<CreateTokenMintContext>,
        decimals: u8,
        freeze_authority: Pubkey,
    ) -> Result<()> {
        // Note: Initialization for mint handled manually 
        // As Poseidon's transpiler does not support initializeMint yet.

        Ok(())
    }
    pub fn mint(ctx: Context<MintContext>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        );
        mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateTokenMintContext<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    // Note: Poseidon's transpiler does not support initializeMint yet,
    // so this code is done manually using Anchor's InitializeMint.
    // init,
    // payer = mint_authority,
    // mint::decimals = decimals,
    // mint::authority = mint_authority.key(), this code is added manually
    #[account(
        init,
        payer = mint_authority,
        mint::decimals = decimals,
        mint::authority = mint_authority.key(),
    )]
    pub mint_account: Account<'info, Mint>,
    // Token Program and System Program is added manually as Poseidon does not support it yet using initializeMint
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct MintContext<'info> {
    #[account(
        init_if_needed,
        payer = mint_authority,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
