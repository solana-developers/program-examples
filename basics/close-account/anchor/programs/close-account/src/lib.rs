use anchor_lang::prelude::*;
mod instructions;
mod state;
use instructions::*;

declare_id!("7vVv26XfvWz8stD4WMnS9KtgLFQq41XCC5sY4WtVjkgv");

#[program]
pub mod close_account_program {
    use super::*;

    pub fn create_user(ctx: Context<CreateUserContext>, name: String) -> Result<()> {
        create_user::create_user(ctx, name)
    }

    pub fn close_user(ctx: Context<CloseUserContext>) -> Result<()> {
        close_user::close_user(ctx)
    }
}
