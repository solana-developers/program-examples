use steel::*;
use crate::prelude::*;

pub fn create_token(
    signer: Pubkey, 
    mint: Pubkey, 
    data: Token
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, true),
            AccountMeta::new(metadata_pda(&mint).0, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false)
        ],         
        data: CreateToken { data }.to_bytes()
    }
}
