use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("4fQVnLWKKKYxtxgGn7Haw8v2g2Hzbu8K61JvWKvqAi7W");

#[program]
pub mod transfer_sol {
    use super::*;

    pub fn transfer_sol_with_cpi(context: Context<TransferSolWithCpiAccountConstraints>, amount: u64) -> Result<()> {
        system_program::transfer(
            CpiContext::new(
                context.accounts.system_program.key(),
                system_program::Transfer {
                    from: context.accounts.payer.to_account_info(),
                    to: context.accounts.recipient.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    // Directly modifying lamports is only possible if the program is the owner of the account
    pub fn transfer_sol_with_program(
        context: Context<TransferSolWithProgramAccountConstraints>,
        amount: u64,
    ) -> Result<()> {
        **context.accounts.payer.try_borrow_mut_lamports()? -= amount;
        **context.accounts.recipient.try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferSolWithCpiAccountConstraints<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSolWithProgramAccountConstraints<'info> {
    /// CHECK: Use owner constraint to check account is owned by our program
    #[account(
        mut,
        owner = id() // value of declare_id!()
    )]
    payer: UncheckedAccount<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
}
