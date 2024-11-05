use anchor_lang::prelude::*;
declare_id!("GsGBeoB6fFTWfUrHhKYTjtXvuiKCC7shhhQqXeQsTLJ2");
#[program]
pub mod favorites {
    use super::*;
    pub fn initialize(
        ctx: Context<InitializeContext>,
        number: u8,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        ctx.accounts.state.owner = ctx.accounts.user.key();
        ctx.accounts.state.number = number;
        ctx.accounts.state.color = color;
        ctx.accounts.state.hobbies = hobbies;
        ctx.accounts.state.bump = ctx.bumps.state;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 130,
        seeds = [b"favorites",
        user.key().as_ref()],
        bump,
    )]
    pub state: Account<'info, FavoritesState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct FavoritesState {
    pub owner: Pubkey,
    pub number: u8,
    pub color: String,
    pub hobbies: Vec<String>,
    pub bump: u8,
}
