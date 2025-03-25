use crate::prelude::*;
use steel::*;

pub fn create_token(
    signer: Pubkey,
    mint: Pubkey,
    name: [u8; 32],
    symbol: [u8; 8],
    uri: [u8; 128],
    decimals: u8,
) -> Instruction {
    // Fetch PDA of the token metadata account.
    let metadata_pda = Pubkey::find_program_address(
        &[METADATA, mpl_token_metadata::ID.as_ref(), mint.as_ref()],
        &mpl_token_metadata::ID,
    );

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, true),
            AccountMeta::new(metadata_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
        ],
        data: CreateToken {
            name,
            symbol,
            uri,
            decimals,
        }
        .to_bytes(),
    }
}
