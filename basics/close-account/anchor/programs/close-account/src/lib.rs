use anchor_lang::prelude::*;

mod instructions;
mod state;
use instructions::*;

declare_id!("9SWqhEABWnKXTPvSLc4aQAJyESVxtRvYBvwF2WuBy7jf");

#[program]
pub mod close_account_program {
    use super::*;

    pub fn create_user(ctx: Context<CreateUserContext>, args: CreateUserArgs) -> Result<()> {
        create_user::create_user(ctx, args)
    }

    pub fn close_user(ctx: Context<CloseUserContext>) -> Result<()> {
        close_user::close_user(ctx)
    }
}
