use anchor_lang::prelude::*;

declare_id!("FbeHkUEevbhKmdk5FE5orcTaJkCYn5drwZoZXaxQXXNn");

#[program]
pub mod cnft_burn {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
