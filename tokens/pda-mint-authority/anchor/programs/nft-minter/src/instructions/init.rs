use anchor_lang::prelude::*;

use crate::state::MintAuthorityPda;

pub fn init(ctx: Context<Init>) -> Result<()> {
    ctx.accounts.mint_authority.set_inner(MintAuthorityPda {
        bump: *ctx.bumps.get("mint_authority").expect("Bump not found."),
    });
    Ok(())
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init,
        payer = payer,
        space = MintAuthorityPda::SIZE,
        seeds = [ MintAuthorityPda::SEED_PREFIX.as_bytes() ],
        bump
    )]
    pub mint_authority: Account<'info, MintAuthorityPda>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
