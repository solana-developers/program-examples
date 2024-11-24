use anchor_lang::prelude::*;
declare_id!("84mLf5VZKf58tQ1VkUtsthxuR8fSeDLv8ZKemANC53oF");
#[program]
pub mod hello_solana {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext {}
