use steel::*;
use crate::prelude::*;

pub fn prepare_fixed_bytes<const N: usize>(input: &str) -> [u8; N] {
    let mut result = [0u8; N];
    let bytes = input.as_bytes();
    let copy_len = bytes.len().min(N);
    if bytes.len() > N {
        solana_program::msg!("Warning: Input truncated from {} to {} bytes", bytes.len(), N);
    }
    result[..copy_len].copy_from_slice(&bytes[..copy_len]);
    result
}

pub fn create_token(
    payer: Pubkey,
    token_name: String,
    token_symbol: String,
    token_uri: String
) -> Instruction {
    let (mint_pda, bump) = Pubkey::find_program_address(
        &[MintAuthorityPda::SEED_PREFIX.as_bytes()], 
        &crate::ID
    );
    let (metadata_pda, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint_pda.as_ref()
        ],
        &mpl_token_metadata::ID
    );
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),                    // Payer is signer and writable
            AccountMeta::new(mint_pda, false),               // Mint account is writable
            AccountMeta::new(mint_pda, false),              // Mint PDA needs to be writable for initialization
            AccountMeta::new(metadata_pda, false),           // Metadata account is writable
            AccountMeta::new_readonly(spl_token::ID, false), // Token program is readonly
            AccountMeta::new_readonly(mpl_token_metadata::ID, false), // Metadata program is readonly
            AccountMeta::new_readonly(system_program::ID, false),    // System program is readonly
            AccountMeta::new_readonly(sysvar::rent::ID, false),     // Rent is readonly
        ],
        data: CreateToken {
            token_name: prepare_fixed_bytes::<32>(&token_name),
            token_symbol: prepare_fixed_bytes::<10>(&token_symbol),
            token_uri: prepare_fixed_bytes::<64>(&token_uri),
            bump
        }.to_bytes(),
    }
}

pub fn mint_token(payer: Pubkey, amount: u64) -> Instruction {
    let (mint_pda, _bump) = Pubkey::find_program_address(
        &[MintAuthorityPda::SEED_PREFIX.as_bytes()], 
        &crate::ID
    );
    let associated_token_account = spl_associated_token_account::get_associated_token_address(&payer, &mint_pda);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),                   // Payer signs and is writable
            AccountMeta::new(mint_pda, false),              // Mint account needs to be writable for minting
            AccountMeta::new(associated_token_account, false), // Token account is writable
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: MintToken {
            amount
        }.to_bytes(),
    }
}
