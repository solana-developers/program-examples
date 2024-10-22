use steel::*;

use crate::prelude::*;

pub fn create_token(
    payer: PubKey,
    mint_authority: PubKey,
    token_name: String,
    token_symbol: String,
    token_uri: String,
) -> Instruction {
    let token_name.bytes: [u8; 32] = token_name.as_bytes().try_into().expect("token_name must be 32 bytes");
    let token_symbol.bytes: [u8; 8] = token_symbol.as_bytes().try_into().expect("token_symbol must be 32 bytes");
    let token_uri.bytes: [u8; 64] = token_uri.as_bytes().try_into().expect("token_uri must be 32 bytes");

    let mint_pda = PubKey::find_program_address(&[b"mint"], &crate::ID);
    let metadata_pda = PubKey::find_program_address();
}
