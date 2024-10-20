use anchor_lang::prelude::*;
declare_id!("6SoSn3xSXpLnJeys6p5ChaoUNdAv7rA4SCdxCanK2zjB");
#[program]
pub mod hello_world_program {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        msg!("hello world");
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext {}
