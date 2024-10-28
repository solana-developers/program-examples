mod init_rent_vault;
mod create_new_account;

use init_rent_vault::*;
use create_new_account::*;

use pda_rent_payer_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix ,data) = parse_instruction(&pda_rent_payer_api::ID, program_id, data)?;

    match ix {
        PdaRentPayerInstruction::InitializeRentVault => process_initialize_vault(accounts, data)?,
        PdaRentPayerInstruction::CreateNewAccount => process_create_account(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
