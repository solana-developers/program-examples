use steel::*;

use crate::prelude::*;

pub fn create(
    payer: Pubkey,
    token_name: String,
    token_symbol: String,
    token_uri: String,
) -> Instruction {
    let token_name_bytes: [u8; 32] = token_name
        .as_bytes()
        .try_into()
        .expect("String wrong length, expected 32 bytes");
    let token_symbol_bytes: [u8; 8] = token_symbol
        .as_bytes()
        .try_into()
        .expect("String wrong length, expected 32 bytes");
    let token_uri_bytes: [u8; 64] = token_uri
        .as_bytes()
        .try_into()
        .expect("String wrong length, expected 32 bytes");

    let mint_pda = Pubkey::find_program_address(&[MINT, MINT_NOISE.as_slice()], &crate::ID);
    let metadata_pda = Pubkey::find_program_address(
        &[
            METADATA,
            mpl_token_metadata::ID.as_ref(),
            mint_pda.0.as_ref(),
        ],
        &mpl_token_metadata::ID,
    );
    let mint_authority_pda = mint_authority_pda();

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(mint_pda.0, false),
            AccountMeta::new(mint_authority_pda.0, false),
            AccountMeta::new(metadata_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
        ],
        data: Create {
            token_name: token_name_bytes,
            token_symbol: token_symbol_bytes,
            token_uri: token_uri_bytes,
            mint_authority_bump: mint_authority_pda.1,
            mint_bump: mint_pda.1,
        }
        .to_bytes(),
    }
}
pub fn mint(
    signer: Pubkey,
    mint: Pubkey,
    to: Pubkey,
    authority: Pubkey,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, false),
            AccountMeta::new(to, false),
            AccountMeta::new(authority, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: Mint {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
