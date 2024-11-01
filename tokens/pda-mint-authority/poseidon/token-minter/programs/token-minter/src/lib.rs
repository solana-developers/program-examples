use anchor_lang::prelude::*;

declare_id!("2Ry3iUWABuQv8PTjgPwaM1CFHB8D8CtuX6EVzYXQ3PvE");

#[program]
pub mod token_minter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
