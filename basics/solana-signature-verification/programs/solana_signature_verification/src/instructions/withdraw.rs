use crate::errors::*;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use anchor_lang::solana_program::sysvar;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

pub fn withdraw_handler(ctx: Context<Withdraw>, id: String) -> Result<()> {
    let price_update = &mut ctx.accounts.price_update;
    let maximum_age: u64 = 30;
    let feed_id: [u8; 32] = get_feed_id_from_hex(&id)?;
    let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;

    msg!(
        "The price is ({} ± {}) * 10^{}",
        price.price,
        price.conf,
        price.exponent
    );
    let actual_price: i128 = (price.price as i128)
        .checked_mul(10i128.pow(price.exponent.unsigned_abs()))
        .ok_or(ProgramError::InvalidArgument)?;

    let escrow_state = &ctx.accounts.escrow_account;
    let unlock_price = escrow_state.unlock_price as i128;
    msg!("Current feed result is {}!", actual_price);
    msg!("Unlock price is {}", escrow_state.unlock_price);

    if actual_price > unlock_price {
        // Subtract the escrow amount from the escrow account's
        **escrow_state.to_account_info().try_borrow_mut_lamports()? = escrow_state
            .to_account_info()
            .lamports()
            .checked_sub(escrow_state.escrow_amount)
            .ok_or(ProgramError::InsufficientFunds)?;

        // Add the escrow amount to the user's account
        **ctx
            .accounts
            .user
            .to_account_info()
            .try_borrow_mut_lamports()? = ctx
            .accounts
            .user
            .to_account_info()
            .lamports()
            .checked_add(escrow_state.escrow_amount)
            .ok_or(ProgramError::InvalidArgument)?;
    } else {
        return Err(error!(EscrowErrorCode::InvalidWithdrawalRequest));
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(id:String)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // The escrow account that holds the SOL, initialized with a PDA
    #[account(
        mut,
        seeds = [ESCROW_SEED, user.key().as_ref()],
        bump,
        close = user
    )]
    pub escrow_account: Account<'info, EscrowState>,
    pub price_update: Account<'info, PriceUpdateV2>,
    pub system_program: Program<'info, System>,
    /// CHECK: Safe because it's a sysvar account
    #[account(address = sysvar::instructions::ID)]
    pub instructions: AccountInfo<'info>,
}
