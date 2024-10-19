use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

use crate::instructions::EscrowInstruction;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = EscrowInstruction::unpack(instruction_data)?;

    match instruction {
        EscrowInstruction::InitEscrow { amount } => {
            msg!("Instruction: InitEscrow");
            crate::instructions::init_escrow::init_escrow(accounts, amount, program_id)
        }
        EscrowInstruction::Exchange { amount } => {
            msg!("Instruction: Exchange");
            crate::instructions::exchange::exchange(accounts, amount, program_id)
        }
    }
}