use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::{
    sysvar::instructions::{load_instruction_at_checked, ID as IX_ID},
};
use crate::errors::ProgramErrorCode;

use crate::utils::ed25519::verify_ed25519_ix;

#[derive(Accounts)]
pub struct Ed25519Example<'info> {
    /// The signer of the message
    #[account(mut)]
    pub signer: Signer<'info>,
    
    /// Instruction sysvar account needed for ed25519 verification
    /// CHECK: This is the instructions sysvar
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,
}

impl<'info> Ed25519Example<'info> {
    pub fn verify_signature(
        &self,
        message: [u8; 64],
        admin_pubkey_bytes: [u8; 32],
        signature: [u8; 64],
    ) -> Result<()> {        
        // Get the Ed25519Program instruction which should be first (index 0)
        let ix: Instruction = anchor_lang::solana_program::sysvar::instructions::load_instruction_at_checked(
            0,
            &self.ix_sysvar
        )?;

        // Verify the signature
        verify_ed25519_ix(&ix, &admin_pubkey_bytes, &message, &signature)?;

        Ok(())
    }
}
