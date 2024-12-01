mod consts;
mod error;
mod instructions;
mod state;

use instructions::*;
use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(program_id, program_id, data)?;

    match ix {
        SteelInstruction::CreateAmm => CreateAmm::process(program_id, accounts, data)?,
        SteelInstruction::CreatePool => CreatePool::process(program_id, accounts)?,
        SteelInstruction::DepositLiquidity => {
            DepositLiquidity::process(program_id, accounts, data)?
        }
        SteelInstruction::SwapExactTokensForTokens => {
            SwapExactTokensForTokens::process(program_id, accounts, data)?
        }
        SteelInstruction::WithdrawLiquidity => {
            WithdrawLiquidity::process(program_id, accounts, data)?
        }
    }

    Ok(())
}
