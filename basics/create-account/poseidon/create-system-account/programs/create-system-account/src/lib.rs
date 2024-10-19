use anchor_lang::prelude::*;

declare_id!("J3h2xRJr7i3dUiLsPu9ZhFGKkNnnxvWAvRNXdKUx5wvi");

#[program]
pub mod create_system_account {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
