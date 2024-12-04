use steel::*;

use crate::prelude::*;

pub fn create(
    payer: Pubkey,
    mint: Pubkey,
    token_name: [u8; 32],
    token_symbol: [u8; 8],
    token_uri: [u8; 64],
) -> Instruction {
    let metadata_pda = Pubkey::find_program_address(
        &[METADATA, mpl_token_metadata::ID.as_ref(), mint.as_ref()],
        &mpl_token_metadata::ID,
    );

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(mint, true),
            AccountMeta::new(metadata_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
        ],
        data: Create {
            token_name,
            token_symbol,
            token_uri,
        }
        .to_bytes(),
    }
}
pub fn mint(
    mint_authority: Pubkey,
    recipient: Pubkey,
    mint: Pubkey,
    associated_token_account: Pubkey,
    quantity: u64,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(mint_authority, true),
            AccountMeta::new(recipient, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(associated_token_account, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Mint {
            quantity: quantity.to_le_bytes(),
        }
        .to_bytes(),
    }
}
