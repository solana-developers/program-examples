use {
    borsh::BorshDeserialize,
    solana_program::{
        account_info::AccountInfo, 
        entrypoint::ProgramResult, 
        program_error::ProgramError, 
    },
};

use crate::instructions::{ create_mint, mint_to };
use crate::state::mint::{ TokenMetadata, MintTokenTo };


pub fn process_instruction(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    match TokenMetadata::try_from_slice(instruction_data) {
        Ok(token_metadata_instruction) => return create_mint::create_mint(
                accounts,
                token_metadata_instruction.title,
                token_metadata_instruction.symbol,
                token_metadata_instruction.uri,
            ),
        Err(_) => {},
    };

    match MintTokenTo::try_from_slice(instruction_data) {
        Ok(mint_to) => return mint_to::mint_to(
                accounts,
                mint_to.amount
            ),
        Err(_) => {},
    };

    Err(ProgramError::InvalidInstructionData)
}