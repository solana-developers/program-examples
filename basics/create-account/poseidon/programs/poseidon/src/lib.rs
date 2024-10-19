use anchor_lang::prelude::*;

declare_id!("HiodPTcV4ZBV8GkqNPRhJKuVoBAxzEQYxK2Mbv9i9vY4");

#[program]
pub mod poseidon {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
