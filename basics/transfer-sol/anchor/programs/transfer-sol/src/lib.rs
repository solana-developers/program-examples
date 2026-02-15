use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("4fQVnLWKKKYxtxgGn7Haw8v2g2Hzbu8K61JvWKvqAi7W");

#[program]
pub mod transfer_sol {
    use super::*;

    pub fn transfer_sol_with_cpi(ctx: Context<TransferSolWithCpi>, amount: u64) -> Result<()> {
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

    // Directly modifying lamports is only possible if the program is the owner of the account
    pub fn transfer_sol_with_program(
        ctx: Context<TransferSolWithProgram>,
        amount: u64,
    ) -> Result<()> {
        // Security invariants:
        // - The source account must authorize the transfer (is_signer)
        // - The source account must be owned by this program (direct lamports mutation)
        // - Prevent under/overflow on lamport arithmetic

        let payer_lamports = ctx.accounts.payer.to_account_info().lamports();
        require!(payer_lamports >= amount, TransferSolError::InsufficientFunds);

        **ctx.accounts.payer.try_borrow_mut_lamports()? = payer_lamports
            .checked_sub(amount)
            .ok_or(TransferSolError::LamportArithmeticOverflow)?;

        let recipient_lamports = ctx.accounts.recipient.to_account_info().lamports();
        **ctx.accounts.recipient.try_borrow_mut_lamports()? = recipient_lamports
            .checked_add(amount)
            .ok_or(TransferSolError::LamportArithmeticOverflow)?;

        Ok(())
    }
}

#[error_code]
pub enum TransferSolError {
    #[msg("Insufficient funds in payer account")]
    InsufficientFunds,

    #[msg("Lamport arithmetic overflow/underflow")]
    LamportArithmeticOverflow,
}

#[derive(Accounts)]
pub struct TransferSolWithCpi<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSolWithProgram<'info> {
    // NOTE: This account must sign the transaction, otherwise *anyone* could drain lamports
    // from program-owned accounts passed into this instruction.
    #[account(
        mut,
        owner = id() // value of declare_id!()
    )]
    payer: Signer<'info>,

    #[account(mut)]
    recipient: SystemAccount<'info>,
}
