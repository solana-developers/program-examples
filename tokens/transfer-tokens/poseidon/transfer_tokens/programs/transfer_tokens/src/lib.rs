use anchor_lang::prelude::*;

declare_id!("4vjYF5HL4xS6sehyeAGhTGExTomDBKPfTWFS5eHj2aqu");

#[program]
pub mod transfer_tokens {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
