use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("7Hm9nsYVuBZ9rf8z9AMUHreZRv8Q4vLhqwdVTCawRZtA");

#[program]
pub mod pda_rent_payer {
    use super::*;

    pub fn init_rent_vault(ctx: Context<InitRentVault>, fund_lamports: u64) -> Result<()> {
        instructions::init_rent_vault::init_rent_vault(ctx, fund_lamports)
    }

    pub fn create_new_account(ctx: Context<CreateNewAccount>) -> Result<()> {
        instructions::create_new_account::create_new_account(ctx)
    }
}
