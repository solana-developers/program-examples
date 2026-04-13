use anchor_lang::prelude::*;
use spl_discriminator::SplDiscriminate;
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;
pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

declare_id!("3ku1ZEGvBEEfhaYsAzBZuecTPEa58ZRhoVqHVGpGxVGi");

#[program]
pub mod abl_token {

    use super::*;

    pub fn init_mint(context: Context<InitMint>, args: InitMintArgs) -> Result<()> {
        context.accounts.init_mint(args)
    }

    pub fn init_config(context: Context<InitConfig>) -> Result<()> {
        context.accounts.init_config(context.bumps.config)
    }

    pub fn attach_to_mint(context: Context<AttachToMint>) -> Result<()> {
        context.accounts.attach_to_mint()
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn tx_hook(context: Context<TxHook>, amount: u64) -> Result<()> {
        context.accounts.tx_hook(amount)
    }

    pub fn init_wallet(context: Context<InitWallet>, args: InitWalletArgs) -> Result<()> {
        context.accounts.init_wallet(args)
    }

    pub fn remove_wallet(context: Context<RemoveWallet>) -> Result<()> {
        context.accounts.remove_wallet()
    }

    pub fn change_mode(context: Context<ChangeMode>, args: ChangeModeArgs) -> Result<()> {
        context.accounts.change_mode(args)
    }
}
