use anchor_lang::prelude::*;

mod instructions;
use instructions::*;

declare_id!("4evptdGtALCNT8uTxJhbWBRZpBE8w5oNtmgfSyfQu7td");

#[program]
pub mod transfer_fee {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        transfer_fee_basis_points: u16,
        maximum_fee: u64,
    ) -> Result<()> {
        process_initialize(ctx, transfer_fee_basis_points, maximum_fee)
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        process_transfer(ctx, amount)
    }

    pub fn harvest<'info>(ctx: Context<'_, '_, 'info, 'info, Harvest<'info>>) -> Result<()> {
        process_harvest(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        process_withdraw(ctx)
    }

    pub fn update_fee(
        ctx: Context<UpdateFee>,
        transfer_fee_basis_points: u16,
        maximum_fee: u64,
    ) -> Result<()> {
        process_update_fee(ctx, transfer_fee_basis_points, maximum_fee)
    }
}
