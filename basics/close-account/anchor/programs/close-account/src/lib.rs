use anchor_lang::prelude::*;
mod instructions;
mod state;
use instructions::*;

declare_id!("99TQtoDdQ5NS2v5Ppha93aqEmv3vV9VZVfHTP5rGST3c");

#[program]
pub mod close_account_program {
    use super::*;

    pub fn create_user(context: Context<CreateUserContext>, name: String) -> Result<()> {
        create_user::handle_create_user(context, name)
    }

    pub fn close_user(context: Context<CloseUserContext>) -> Result<()> {
        close_user::handle_close_user(context)
    }
}
