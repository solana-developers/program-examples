mod error;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;
use spl_discriminator::SplDiscriminate;
use spl_transfer_hook_interface::instruction::{
    ExecuteInstruction, InitializeExtraAccountMetaListInstruction,
};

declare_id!("FjcHckEgXcBhFmSGai3FRpDLiT6hbpV893n8iTxVd81g");

#[program]
pub mod transfer_switch {
    use super::*;

    pub fn configure_admin(mut context: Context<ConfigureAdmin>) -> Result<()> {
        handle_is_admin(&mut context.accounts)?;
        handle_configure_admin(&mut context.accounts)
    }

    #[instruction(discriminator = InitializeExtraAccountMetaListInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn initialize_extra_account_metas_list(
        mut context: Context<InitializeExtraAccountMetas>,
    ) -> Result<()> {
        handle_initialize_extra_account_metas_list(&mut context.accounts, context.bumps)
    }

    pub fn switch(mut context: Context<Switch>, on: bool) -> Result<()> {
        handle_switch(&mut context.accounts, on)
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(mut context: Context<TransferHook>, _amount: u64) -> Result<()> {
        handle_assert_is_transferring(&mut context.accounts)?;
        handle_assert_switch_is_on(&mut context.accounts)
    }
}
