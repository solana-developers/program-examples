use anchor_lang::prelude::*;
declare_id!("9Ewc72Ju79bJ3qEaASXtNjAQphdJcHCNpELcwTFVtXnM");
#[program]
pub mod favorites_program {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        ctx.accounts.state.favorite = 0;
        Ok(())
    }
    pub fn add(ctx: Context<AddContext>) -> Result<()> {
        ctx.accounts.state.favorite = ctx.accounts.state.favorite + 1;
        Ok(())
    }
    pub fn remove(ctx: Context<RemoveContext>) -> Result<()> {
        ctx.accounts.state.favorite = ctx.accounts.state.favorite - 1;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 17, seeds = [b"favorites"], bump)]
    pub state: Account<'info, FavoriteState>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct AddContext<'info> {
    #[account(mut, seeds = [b"favorites"], bump)]
    pub state: Account<'info, FavoriteState>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RemoveContext<'info> {
    #[account(mut, seeds = [b"favorites"], bump)]
    pub state: Account<'info, FavoriteState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct FavoriteState {
    pub favorite: i64,
    pub bump: u8,
}
