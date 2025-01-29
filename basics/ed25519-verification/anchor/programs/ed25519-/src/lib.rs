use anchor_lang::prelude::*;
use anchor_lang::solana_program::ed25519_program;

declare_id!("Ed25519CustodiaLXXXXXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod ed25519_custodial {
    use super::*;

    pub fn transfer(
        ctx: Context<Transfer>,
        signature: [u8; 64],
        public_key: [u8; 32],
        message: Vec<u8>,
        amount: u64,
    ) -> Result<()> {
        // Verify Ed25519 signature
        let verification_instruction = ed25519_program::instruction::new_ed25519_instruction(
            &public_key,
            &message,
            &signature,
        );

        // Invoke the Ed25519 program to verify the signature
        solana_program::program::invoke(
            &verification_instruction,
            &[ctx.accounts.ed25519_program.to_account_info()],
        )?;

        msg!("Signature verification successful!");

        // Transfer funds
        **ctx.accounts.custodial_account.try_borrow_mut_lamports()? = ctx
            .accounts
            .custodial_account
            .lamports()
            .checked_sub(amount)
            .ok_or(ProgramError::InsufficientFunds)?;

        **ctx.accounts.recipient.try_borrow_mut_lamports()? = ctx
            .accounts
            .recipient
            .lamports()
            .checked_add(amount)
            .ok_or(ProgramError::Overflow)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub custodial_account: AccountInfo<'info>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub signer: Signer<'info>,
    /// CHECK: This is the Ed25519 program ID
    pub ed25519_program: AccountInfo<'info>,
} 