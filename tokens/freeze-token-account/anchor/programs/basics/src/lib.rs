use anchor_lang::prelude::*;
use anchor_spl::token;

declare_id!("6DmL2kW79bwMm6SDoyoxnEGGkx8XKF9Z7mx5USB82qof");

#[program]
pub mod anchori {

    use super::*;

    pub fn freeze_token_account(ctx: Context<CreateMintAndFreezeTokenAccount>) -> Result<()> {
        token::freeze_account(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::FreezeAccount {
                account: ctx.accounts.associated_token_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        ))?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMintAndFreezeTokenAccount<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint_authority.key(),
        mint::freeze_authority = mint_authority.key(),
    )]
    pub mint_account: Account<'info, token::Mint>,
    pub mint_authority: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
}
