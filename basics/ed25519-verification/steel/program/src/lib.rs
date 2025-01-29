use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    ed25519_program,
};
use steel::steel_program;

#[steel_program]
pub mod ed25519_custodial {
    use super::*;

    pub fn transfer(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        signature: [u8; 64],
        public_key: [u8; 32],
        message: Vec<u8>,
        amount: u64,
    ) -> ProgramResult {
        let custodial_account = &accounts[0];
        let recipient = &accounts[1];
        let signer = &accounts[2];
        let ed25519_program_id = &accounts[3];

        // Verify this is the expected Ed25519 program
        if ed25519_program_id.key != &ed25519_program::id() {
            return Err(ProgramError::InvalidArgument);
        }

        // Verify the Ed25519 signature
        let verification_instruction = ed25519_program::instruction::new_ed25519_instruction(
            &public_key,
            &message,
            &signature,
        );

        // Invoke the Ed25519 program to verify the signature
        solana_program::program::invoke(
            &verification_instruction,
            &[ed25519_program_id.clone()],
        )?;

        msg!("Signature verification successful!");

        // Transfer funds
        **custodial_account.try_borrow_mut_lamports()? = custodial_account
            .lamports()
            .checked_sub(amount)
            .ok_or(ProgramError::InsufficientFunds)?;

        **recipient.try_borrow_mut_lamports()? = recipient
            .lamports()
            .checked_add(amount)
            .ok_or(ProgramError::Overflow)?;

        Ok(())
    }
} 