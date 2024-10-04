#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("4fQVnLWKKKYxtxgGn7Haw8v2g2Hzbu8K61JvWKvqAi7W");

#[program]
pub mod transfer_sol {
    use super::*;

    /// Transfer SOL using Cross-Program Invocation (CPI)
    pub fn transfer_sol_with_cpi(ctx: Context<TransferSolWithCpi>, amount: u64) -> Result<()> {
        // Check for sufficient balance before transferring
        let payer_balance: u64 = **ctx.accounts.payer.try_borrow_lamports()?;
        require!(payer_balance >= amount, CustomError::InsufficientFunds);

        // CPI-based transfer using the system program's transfer function
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.recipient.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    /// Direct SOL transfer by modifying the lamports of accounts
    /// This method is only possible when the program owns the accounts being modified.
    pub fn transfer_sol_with_program(
        ctx: Context<TransferSolWithProgram>,
        amount: u64,
    ) -> Result<()> {
        // Check if the payer has sufficient funds for the transfer
        let payer_balance = **ctx.accounts.payer.try_borrow_lamports()?;
        require!(payer_balance >= amount, CustomError::InsufficientFunds);

        **ctx.accounts.payer.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.recipient.try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferSolWithCpi<'info> {
    /// The payer account which signs the transaction and sends SOL
    #[account(mut)]
    payer: Signer<'info>,

    /// The recipient account that receives SOL
    #[account(mut)]
    recipient: SystemAccount<'info>,

    /// Reference to the system program for CPI
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSolWithProgram<'info> {
    /// CHECK: Ensures the payer is owned by the program before modifying lamports
    #[account(
        mut,
        owner = id() // Ensures this account is owned by the program
    )]
    payer: UncheckedAccount<'info>,

    /// The recipient account that receives SOL
    #[account(mut)]
    recipient: SystemAccount<'info>,
}

/// Custom errors for better handling of specific issues
#[error_code]
pub enum CustomError {
    #[msg("Insufficient funds for the transfer.")]
    InsufficientFunds,
}
