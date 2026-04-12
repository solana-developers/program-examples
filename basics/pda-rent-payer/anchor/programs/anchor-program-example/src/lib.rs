use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;

declare_id!("7Hm9nsYVuBZ9rf8z9AMUHreZRv8Q4vLhqwdVTCawRZtA");

#[program]
pub mod pda_rent_payer {
    use super::*;

    pub fn init_rent_vault(context: Context<InitRentVaultAccountConstraints>, fund_lamports: u64) -> Result<()> {
        init_rent_vault::handle_init_rent_vault(context, fund_lamports)
    }

    pub fn create_new_account(context: Context<CreateNewAccountAccountConstraints>) -> Result<()> {
        create_new_account::handle_create_new_account(context)
    }
}
