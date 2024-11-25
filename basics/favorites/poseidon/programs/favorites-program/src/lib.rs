use anchor_lang::prelude::*;
declare_id!("HMYL9ABJz8fpw6XUnkRAYVsXor4JxosiZqHBd38ZgCqS");
#[program]
pub mod favorites_program {
    use super::*;
    pub fn set_favorites(
        ctx: Context<SetFavoritesContext>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        ctx.accounts.favorites.number = number;
        ctx.accounts.favorites.color = color;
        ctx.accounts.favorites.hobbies = hobbies;
        ctx.accounts.favorites.bump = ctx.bumps.favorites;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct SetFavoritesContext<'info> {
    #[account(
        init,
        payer = payer,
        space = 87,
        seeds = [b"favorites",
        payer.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Favorites {
    pub number: u64,
    pub color: String,
    pub hobbies: Vec<String>,
    pub bump: u8,
}
