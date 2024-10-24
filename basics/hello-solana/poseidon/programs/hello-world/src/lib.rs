use anchor_lang::prelude::*;
declare_id!("6SoSn3xSXpLnJeys6p5ChaoUNdAv7rA4SCdxCanK2zjB");
#[program]
pub mod hello_world_program {
    use super::*;
    pub fn hello_solana(ctx: Context<HelloSolanaContext>) -> Result<()> {
        msg!("Hello solana");
        Ok(())
    }
}
#[derive(Accounts)]
pub struct HelloSolanaContext {}
