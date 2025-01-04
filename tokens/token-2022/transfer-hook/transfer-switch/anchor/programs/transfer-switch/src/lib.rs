mod error;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("FjcHckEgXcBhFmSGai3FRpDLiT6hbpV893n8iTxVd81g");

#[program]
pub mod transfer_switch {
    use super::*;

    pub fn configure_admin(ctx: Context<ConfigureAdmin>) -> Result<()> {
        ctx.accounts.is_admin()?;
        ctx.accounts.configure_admin()
    }

    #[interface(spl_transfer_hook_interface::initialize_extra_account_meta_list)]
    pub fn initialize_extra_account_metas_list(
        ctx: Context<InitializeExtraAccountMetas>,
    ) -> Result<()> {
        ctx.accounts.initialize_extra_account_metas_list(ctx.bumps)
    }

    pub fn switch(ctx: Context<Switch>, on: bool) -> Result<()> {
        ctx.accounts.switch(on)
    }

    #[interface(spl_transfer_hook_interface::execute)]
    pub fn transfer_hook(ctx: Context<TransferHook>) -> Result<()> {
        ctx.accounts.assert_is_transferring()?;
        ctx.accounts.assert_switch_is_on()
    }
}
