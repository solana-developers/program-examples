use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey},
};

use crate::instructions::{
    create::{create_token, CreateTokenArgs},
    mint::{mint_to, MintToArgs},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum SplMinterIntstruction {
    Create(CreateTokenArgs),
    Mint(MintToArgs),
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SplMinterIntstruction::try_from_slice(instruction_data)?;

    match instruction {
        SplMinterIntstruction::Create(args) => create_token(accounts, args),
        SplMinterIntstruction::Mint(args) => mint_to(accounts, args),
    }
}
