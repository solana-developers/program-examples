use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::associated_token;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Debug)]
#[account]
pub struct Counter {
    count: u64,
}

pub fn initialize_counter_handler(mut ctx: Context<InitializeCounter>, mut seed: u8) -> Result<()> {
    let mut counter = &mut ctx.accounts.counter;
    let mut payer = &mut ctx.accounts.payer;

    Ok(())
}

pub fn increment_handler(mut ctx: Context<Increment>) -> Result<()> {
    let mut counter = &mut ctx.accounts.counter;

    counter.count += 1;

    Ok(())
}

#[derive(Accounts)]
# [instruction (seed : u8)]
pub struct InitializeCounter<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Counter>()
    )]
    pub counter: Box<Account<'info, Counter>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Box<Account<'info, Counter>>,
}

#[program]
pub mod counter_seahorse {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounter>, seed: u8) -> Result<()> {
        initialize_counter_handler(ctx, seed)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        increment_handler(ctx)
    }
}
