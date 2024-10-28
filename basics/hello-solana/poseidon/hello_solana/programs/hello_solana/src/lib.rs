use anchor_lang::prelude::*;

declare_id!("68tzQa3yyDTM5czVh4YK6QqEwbGirMa8SYBwFnovmh5c");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
