use anchor_lang::prelude::*;

declare_id!("52rNd2KDuqHaxs2vEFfEjH2zwKScA2B9AyW8F2fAcca8");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn hello(_ctx: Context<Hello>) -> Result<()> {
        msg!("Hello, Solana!");

        msg!("Our program's Program ID: {}", &id());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}
