use anchor_lang::prelude::*;
declare_id!("BreVFi2U3pUegY96xP5JMviUuxL5x6bRnnbsztb262vQ");
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
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 87,
        seeds = [b"favorites",
        payer.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Favorites {
    pub number: u64,
    pub color: String,
    pub hobbies: Vec<String>,
    pub bump: u8,
}
