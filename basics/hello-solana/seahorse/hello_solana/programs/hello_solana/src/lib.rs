use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Debug)]
#[account]
pub struct Counter {
    authority: Pubkey,
    value: u8,
}

pub fn initialize_handler(mut ctx: Context<Initialize>) -> Result<()> {
    let mut authority = &mut ctx.accounts.authority;
    let mut counter = &mut ctx.accounts.counter;
    let mut counter = counter;

    counter.authority = authority.key();

    counter.value = 0;

    msg!("{}", "Hello, Solana from Seahorse!");

    Ok(())
}

pub fn increment_handler(mut ctx: Context<Increment>) -> Result<()> {
    let mut authority = &mut ctx.accounts.authority;
    let mut counter = &mut ctx.accounts.counter;

    counter.value += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        seeds = ["new_delhi_hh".as_bytes().as_ref(), authority.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Counter>()
    )]
    pub counter: Box<Account<'info, Counter>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub counter: Box<Account<'info, Counter>>,
}

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_handler(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        increment_handler(ctx)
    }
}
