use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        mint_to, transfer as transfer_spl, Mint, MintTo, Token, TokenAccount,
        Transfer as TransferSPL,
    },
};
declare_id!("CSqtsYXnt2UfXttszwG6rGFFY7EedJ5kmn4xEyas4LeE");
#[program]
pub mod transfer_tokens_program {
    use super::*;
    pub fn create_token(
        _ctx: Context<CreateTokenContext>,
        _decimals: u8,
        _freeze_authority: Pubkey,
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
    pub fn transfer_tokens(ctx: Context<TransferTokensContext>, amount: u64) -> Result<()> {
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        transfer_spl(cpi_ctx, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateTokenContext<'info> {
    // Note: Poseidon's transpiler does not support initializeMint yet,
    // so this code is done manually using Anchor's InitializeMint.
    // init,
    // payer = mint_authority,
    // mint::decimals = decimals,
    // mint::authority = mint_authority.key(), this code is added manually
    #[account(
        init,
        payer = payer,
        mint::decimals = decimals,
        mint::authority = payer.key(),
    )]
    pub mint_account: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    // Token Program and System Program is added manually as Poseidon does not support it yet using initializeMint
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct MintContext<'info> {
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = mint_authority,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct TransferTokensContext<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_account,
        associated_token::authority = sender,
    )]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
