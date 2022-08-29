use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::associated_token;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub fn hello_handler(mut ctx: Context<Hello>) -> Result<()> {
    let mut signer = &mut ctx.accounts.signer;

    msg!("{}", "Hello, Solana from Seahorse!");

    msg!(
        "{}",
        format!("This is the public key of the signer: {}", signer.key())
    );

    Ok(())
}

#[derive(Accounts)]
pub struct Hello<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[program]
pub mod hello_solana {
    use super::*;

    pub fn hello(ctx: Context<Hello>) -> Result<()> {
        hello_handler(ctx)
    }
}
