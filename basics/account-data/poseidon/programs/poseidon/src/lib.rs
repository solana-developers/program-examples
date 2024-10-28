use anchor_lang::prelude::*;

declare_id!("3CCD5L37Ht1rZjV6LQxDqNkZ3g6ownWC1QvDSca7M2Ti");

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
