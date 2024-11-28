use anchor_lang::prelude::*;
declare_id!("H4mwUctvTzbQ7bgxceko6eoi3qYH9vmFqwMoPVQ9vf5T");
#[program]
pub mod favorites {
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
        Ok(())
    }
}
#[derive(Accounts)]
pub struct SetFavoritesContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = 344,
        seeds = [b"favorites",
        user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, FavoritesAccount>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct FavoritesAccount {
    pub number: u64,
    pub color: String,
    pub hobbies: Vec<String>,
}
