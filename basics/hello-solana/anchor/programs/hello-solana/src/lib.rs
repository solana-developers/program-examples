#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("2nYa9FRtxLnaGa5agENEE1ehy6Tr2HnyziwG7ynnyhPC");

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
