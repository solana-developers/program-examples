use anchor_lang::prelude::*;

declare_id!("9zHaE8x381ZRg78MiEykQbgs1jDcLT52dooPrEj3PN9Z");

#[program]
pub mod create_tokens {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
