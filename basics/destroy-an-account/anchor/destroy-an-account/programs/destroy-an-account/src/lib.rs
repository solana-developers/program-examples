use anchor_lang::prelude::*;

mod instructions;
mod state;
use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod destroy_an_account {
    use super::*;

    pub fn create_user(ctx: Context<CreateUserContext>, args: CreateUserArgs) -> Result<()> {
        create_user::create_user(ctx, args)
    }

    pub fn destroy_user(ctx: Context<DestroyUserContext>) -> Result<()> {
        destroy_user::destroy_user(ctx)
    }
}
