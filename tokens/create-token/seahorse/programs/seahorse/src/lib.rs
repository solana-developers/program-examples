use anchor_lang::prelude::*;

declare_id!("3ZckR8obvBASHPqWD6gcWgb8gCdw5UseJ7D1xFzxnZ4B");

#[program]
pub mod seahorse {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
