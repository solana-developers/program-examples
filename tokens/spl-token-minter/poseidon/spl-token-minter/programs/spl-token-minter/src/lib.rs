use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
declare_id!("DmrXSUGWYaqtWg8sbi9JQN48yVZ1y2m7HvWXbND52Mcw");
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
        // here in CpiContext::new, when transpiling from Poseidon new_with_signer was used instead of new
        // so remove this code
        // let cpi_ctx = CpiContext::new_with_signer(
        //     ctx.accounts.token_program.to_account_info(),
        //     MintTo {
        //         mint: ctx.accounts.mint_account.to_account_info(),
        //         to: ctx.accounts.associated_token_account.to_account_info(),
        //         authority: ctx.accounts.mint_authority.to_account_info(),
        //     },
        //     signer,
        // );
        // mint_to(cpi_ctx, amount)?; 
        // and change manually to :
        // let cpi_ctx = CpiContext::new(
        //     ctx.accounts.token_program.to_account_info(),
        //     MintTo {
        //         mint: ctx.accounts.mint_account.to_account_info(),
        //         to: ctx.accounts.associated_token_account.to_account_info(),
        //         authority: ctx.accounts.mint_authority.to_account_info(),
        //     },
        // );
        // mint_to(cpi_ctx, amount)?;


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
    #[account(mut)]     // here (mut) is added manually as Poseidon didn't add it by its own causing error in build
    pub mint_account: Account<'info, Mint>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = mint_authority,   // Explicitly set payer to mint_authority due to Poseidon issue. Here poseidon added payer as mintAuthority but it should be mint_authority
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
