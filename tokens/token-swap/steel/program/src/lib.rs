mod create_amm;
mod create_pool;
mod deposit_liquidity;
mod swap;
mod withdraw_liquidity;
use create_amm::*;
use create_pool::*;
use deposit_liquidity::*;
use swap::*;
use withdraw_liquidity::*;

use steel::*;
use token_swap_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&token_swap_api::ID, program_id, data)?;

    match ix {
        TokenSwapInstruction::CreateAmm => process_create_amm(accounts, data)?,
        TokenSwapInstruction::CreatePool => process_create_pool(accounts, data)?,
        TokenSwapInstruction::DepositLiquidity => process_deposit_liquidity(accounts, data)?,
        TokenSwapInstruction::WithdrawLiquidity => process_withdraw_liquidity(accounts, data)?,
        TokenSwapInstruction::Swap => process_swap(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
