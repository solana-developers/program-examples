use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("4fQVnLWKKKYxtxgGn7Haw8v2g2Hzbu8K61JvWKvqAi7W");

#[program]
pub mod transfer_sol {
    use super::*;

    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        
        msg!("Received request to transfer {:?} lamports from {:?} to {:?}.", 
            amount, &ctx.accounts.payer.key(), &ctx.accounts.recipient.key());
        msg!("  Processing transfer...");

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
        
        msg!("Transfer completed successfully.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferSol<'info> {
    /// CHECK: We're initializing this account via the transfer
    #[account(mut)]
    recipient: AccountInfo<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}