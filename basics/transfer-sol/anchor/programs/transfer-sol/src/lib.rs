use anchor_lang::prelude::*;
use anchor_lang::system_program;


declare_id!("4fQVnLWKKKYxtxgGn7Haw8v2g2Hzbu8K61JvWKvqAi7W");


#[program]
pub mod transfer_sol {
    use super::*;

    pub fn transfer_sol_with_cpi(
        ctx: Context<TransferSolWithCpi>, 
        amount: u64
    ) -> Result<()> {

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

    pub fn transfer_sol_with_program(
        ctx: Context<TransferSolWithProgram>, 
        amount: u64
    ) -> Result<()> {

        **ctx.accounts.payer
            .to_account_info()
            .try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.recipient
            .to_account_info()
            .try_borrow_mut_lamports()? += amount;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferSolWithCpi<'info> {
    #[account(mut)]
    recipient: SystemAccount<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSolWithProgram<'info> {
    /// CHECK: This is just an example, not checking data
    #[account(mut)]
    recipient: UncheckedAccount<'info>,
    /// CHECK: This is just an example, not checking data
    #[account(mut)]
    payer: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}
