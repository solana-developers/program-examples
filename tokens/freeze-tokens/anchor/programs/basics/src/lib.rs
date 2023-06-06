use anchor_lang::prelude::*;
use anchor_spl::token;
use spl_token::instruction::AuthorityType;

declare_id!("6DmL2kW79bwMm6SDoyoxnEGGkx8XKF9Z7mx5USB82qof");

#[program]
pub mod anchor {

    use super::*;

    pub fn mint_and_disable_mint(ctx: Context<CreateToken>) -> Result<()> {
        // Fix the Mint supply
        token::set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::SetAuthority {
                    account_or_mint: ctx.accounts.mint_account.to_account_info(),
                    current_authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            AuthorityType::MintTokens,
            // Disable future minting by setting mint authority to None
            None,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = payer.key(),
    )]
    pub mint_account: Account<'info, token::Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}
