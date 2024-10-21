use anchor_lang::prelude::*;

declare_id!("3cvZMR8oDVXVcxcfuPmBpsEWnGMYh2uomwYohNSJSWwk");

#[program]
pub mod account_data {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
