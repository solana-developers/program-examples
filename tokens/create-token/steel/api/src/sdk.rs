use steel::*;

use crate::prelude::*;

pub fn create(
    payer: Pubkey,
    mint: Pubkey,
    token_name: [u8; 32],
    token_symbol: [u8; 8],
    token_uri: [u8; 64],
    decimals: u8,
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
            decimals,
        }
        .to_bytes(),
    }
}
