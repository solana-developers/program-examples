use crate::{constants::*, error::*, events::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct SetWalletState<'info> {
    #[account(
        init_if_needed,
        payer = authority,
        space = DISCRIMINATOR_SIZE + WalletState::INIT_SPACE,
        seeds = [WALLET_STATE_SEED, wallet.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub wallet_state: Account<'info, WalletState>,

    /// CHECK: Wallet address used for PDA derivation and stored in state
    pub wallet: UncheckedAccount<'info>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [STATE_SEED],
        bump = state.bump,
        constraint = state.authority == authority.key() @ TransferHookError::InvalidAuthority
    )]
    pub state: Account<'info, TransferHookState>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn set_wallet_state(ctx: Context<SetWalletState>, is_frozen: bool) -> Result<()> {
    let wallet_state = &mut ctx.accounts.wallet_state;
    wallet_state.is_frozen = is_frozen;
    wallet_state.bump = ctx.bumps.wallet_state;
    wallet_state.mint = ctx.accounts.mint.key();
    wallet_state.owner = ctx.accounts.wallet.key();

    emit!(WalletStateChanged {
        wallet: ctx.accounts.wallet.key(),
        is_frozen,
        mint: ctx.accounts.mint.key()
    });

    msg!(
        "Wallet state updated for {}: frozen = {}",
        ctx.accounts.wallet.key(),
        is_frozen
    );
    Ok(())
}
