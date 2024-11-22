use anchor_lang::prelude::*;

declare_id!("8krac1be77qraaZQGzHypmngXH6MbftjBmcucLtQLahG");

#[program]
pub mod poseidon {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
