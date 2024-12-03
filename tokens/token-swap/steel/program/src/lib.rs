mod create_amm;
mod create_pool;
mod deposit_liquidity;
mod swap;
mod withdraw;

use create_amm::*;
use create_pool::*;
use deposit_liquidity::*;
use swap::*;
use withdraw::*;

use steel::*;
use token_swap_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&token_swap_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::CreateAmm => process_create_amm(accounts, data)?,
        SteelInstruction::CreatePool => process_create_pool(accounts)?,
        SteelInstruction::DepositLiquidity => process_deposit(accounts, data)?,
        SteelInstruction::SwapExactTokens => process_swap(accounts, data)?,
        SteelInstruction::WithdrawLiquidity => process_withdraw_liquidity(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
