use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod events;
pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("2KKm5dexWn2uw7tPTYs4KYc1R2P111rCu7YUEhZyoLh2");

#[program]
pub mod transfer_hook {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing transfer hook program");
        instructions::initialize::initialize(ctx)
    }

    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        msg!("Creating new token mint");
        instructions::create_mint::create_mint(ctx)
    }

    pub fn set_wallet_state(
        ctx: Context<SetWalletState>,
        is_frozen: bool,
    ) -> Result<()> {
        msg!("Setting wallet state: frozen = {}", is_frozen);
        instructions::set_wallet_state::set_wallet_state(ctx, is_frozen)
    }

    pub fn transfer(
        ctx: Context<TransferTokens>,
        amount: u64,
    ) -> Result<()> {
        msg!("Processing transfer of {} tokens", amount);
        instructions::transfer::transfer_tokens(ctx, amount)
    }
}