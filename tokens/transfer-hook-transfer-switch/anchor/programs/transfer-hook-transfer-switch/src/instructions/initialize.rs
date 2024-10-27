use anchor_lang::prelude::*;
use crate::{state::*, constants::*};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = DISCRIMINATOR_SIZE + TransferHookState::INIT_SPACE,
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, TransferHookState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.authority = ctx.accounts.authority.key();
    state.bump = ctx.bumps.state;
    
    msg!("Transfer hook state initialized with authority: {}", state.authority);
    Ok(())
}
