use anchor_lang::prelude::*;

declare_id!("7GWQBBQmcfjWdnyjkXdxUXeDxp2mW1WsGtMhpsFD8eKN");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
