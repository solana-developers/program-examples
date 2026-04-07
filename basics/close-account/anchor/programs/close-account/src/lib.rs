use anchor_lang::prelude::*;
mod instructions;
mod state;
use instructions::*;

declare_id!("E7Cgyech7DrHz39ctmsPe6xqe65YJKWg1jwQQpmZrtcm");

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
