use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_lang::solana_program::{program::invoke, system_instruction::transfer};
pub fn deposit_handler(ctx: Context<Deposit>, escrow_amount: u64, unlock_price: u64) -> Result<()> {
    msg!("Depositing funds in escrow...");

    let escrow_state = &mut ctx.accounts.escrow_account;
    escrow_state.unlock_price = unlock_price;
    escrow_state.escrow_amount = escrow_amount;
    // transfer instruction to move funds from the user's account to the escrow account
    let transfer_ix = transfer(&ctx.accounts.user.key(), &escrow_state.key(), escrow_amount);

    invoke(
        &transfer_ix,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.escrow_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    msg!(
        "Transfer complete. Escrow will unlock SOL at {}",
        &ctx.accounts.escrow_account.unlock_price
    );

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // Escrow account to store SOL in escrow
    #[account(
        init,
        seeds = [ESCROW_SEED, user.key().as_ref()], // Use a PDA with the user key and a seed to generate the escrow account address
        bump,
        payer = user,
        space = std::mem::size_of::<EscrowState>() + 8
    )]
    pub escrow_account: Account<'info, EscrowState>,

    pub system_program: Program<'info, System>,
    /// CHECK: Safe because it's a sysvar account
    #[account(address = sysvar::instructions::ID)]
    pub instructions: AccountInfo<'info>,
}
