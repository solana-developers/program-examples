use {
    borsh::BorshDeserialize,
    solana_program::{
        account_info::AccountInfo, 
        entrypoint::ProgramResult, 
        program_error::ProgramError, 
        pubkey::Pubkey,
    },
};

use crate::instructions::{ create_token_mint, mint_to_wallet, transfer_to_wallet };
use crate::state::mint_state::{ TokenMetadata, MintTokensTo, TransferTokensTo };


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    match TokenMetadata::try_from_slice(instruction_data) {
        Ok(token_metadata_instruction) => return create_token_mint::create_token_mint(
            program_id,
            accounts,
            token_metadata_instruction.title,
            token_metadata_instruction.symbol,
            token_metadata_instruction.uri,
            token_metadata_instruction.mint_authority_pda_bump,
        ),
        Err(_) => {},
    };

    match MintTokensTo::try_from_slice(instruction_data) {
        Ok(mint_to_wallet_instruction) => return mint_to_wallet::mint_to_wallet(
            program_id,
            accounts,
            mint_to_wallet_instruction.amount,
            mint_to_wallet_instruction.mint_authority_pda_bump,
        ),
        Err(_) => {},
    };

    match TransferTokensTo::try_from_slice(instruction_data) {
        Ok(transfer_to_wallet_instruction) => return transfer_to_wallet::transfer_to_wallet(
            program_id,
            accounts,
            transfer_to_wallet_instruction.amount,
        ),
        Err(_) => {},
    };

    Err(ProgramError::InvalidInstructionData)
}