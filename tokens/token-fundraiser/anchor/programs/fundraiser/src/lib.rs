use anchor_lang::prelude::*;

declare_id!("Eoiuq1dXvHxh6dLx3wh9gj8kSAUpga11krTrbfF5XYsC");

mod constants;
mod error;
mod instructions;
mod state;

pub use constants::*;
use error::*;
use instructions::*;

#[program]
pub mod fundraiser {
    use super::*;

    pub fn initialize(mut context: Context<Initialize>, amount: u64, duration: u16) -> Result<()> {
        handle_initialize(&mut context.accounts, amount, duration, &context.bumps)?;

        Ok(())
    }

    pub fn contribute(mut context: Context<Contribute>, amount: u64) -> Result<()> {
        handle_contribute(&mut context.accounts, amount)?;

        Ok(())
    }

    pub fn check_contributions(mut context: Context<CheckContributions>) -> Result<()> {
        handle_check_contributions(&mut context.accounts)?;

        Ok(())
    }

    pub fn refund(mut context: Context<Refund>) -> Result<()> {
        handle_refund(&mut context.accounts)?;

        Ok(())
    }
}
