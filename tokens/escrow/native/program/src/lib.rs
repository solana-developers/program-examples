mod error;
mod instructions;
mod state;
mod utils;

use {
    borsh::{BorshDeserialize, BorshSerialize},
    instructions::*,
    solana_program::{
        account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
    },
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = EscrowInstruction::try_from_slice(instruction_data)?;

    match instruction {
        EscrowInstruction::MakeOffer(data) => MakeOffer::process(program_id, accounts, data),
        EscrowInstruction::TakeOffer => TakeOffer::process(program_id, accounts),
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum EscrowInstruction {
    MakeOffer(MakeOffer),
    TakeOffer,
}
