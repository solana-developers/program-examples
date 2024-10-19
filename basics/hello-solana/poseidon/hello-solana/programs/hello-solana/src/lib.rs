use anchor_lang::prelude::*;

declare_id!("BHJvP5fFucNNQNTpN8gfq7xTNEhaHxea2e38ab4AzLKr");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
