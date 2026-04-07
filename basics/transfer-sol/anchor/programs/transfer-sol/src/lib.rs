use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("5bz3T8zeTXMGEwVhuFBKmyZk1Wqhfs1K39kfDXnLLmwG");

#[program]
pub mod transfer_sol {
    use super::*;

    pub fn transfer_sol_with_cpi(ctx: Context<TransferSolWithCpi>, amount: u64) -> Result<()> {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.key(),
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
        **ctx.accounts.payer.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.recipient.try_borrow_mut_lamports()? += amount;
        Ok(())
    }
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
    /// CHECK: Use owner constraint to check account is owned by our program
    #[account(
        mut,
        owner = id() // value of declare_id!()
    )]
    payer: UncheckedAccount<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
}
