use steel::*;
use transfer_sol_steel_api::prelude::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&transfer_sol_steel_api::ID, program_id, data)?;

    match ix {
        TransferInstruction::TransferSolWithCpi => TransferSolWithCpi::process(accounts, data),
        TransferInstruction::TransferSolWithProgram => {
            TransferSolWithProgram::process(accounts, data)
        }
    }
}
