use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn hello(ctx: Context<Hello>) -> Result<()> {
        
        msg!("Hello, Solana!");
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}
