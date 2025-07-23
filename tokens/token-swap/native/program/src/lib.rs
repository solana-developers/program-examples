pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

use crate::instructions::{
    process_create_amm, process_create_pool, process_deposit_liquidity,
    process_swap_exact_tokens_for_tokens, process_withdraw_liquidity, AmmInstruction,
};

declare_id!("5tS77fBNSDtMSuyBfizp3bdBCcgmVPuLTKzYpZjgoMjq");
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = AmmInstruction::try_from_slice(instruction_data)?;
    match instruction {
        AmmInstruction::CreateAmm(args) => process_create_amm(program_id, accounts, args),
        AmmInstruction::CreatePool(args) => process_create_pool(program_id, accounts, args),
        AmmInstruction::DepositLiquidity(args) => {
            process_deposit_liquidity(program_id, accounts, args)
        }
        AmmInstruction::SwapExactTokensForToken(args) => {
            process_swap_exact_tokens_for_tokens(program_id, accounts, args)
        }
        AmmInstruction::WithdrawLiquidity(args) => {
            process_withdraw_liquidity(program_id, accounts, args)
        }
    }
}
