mod create_new_account;
mod init_rent_vault;

use create_new_account::*;
use init_rent_vault::*;

use pda_rent_payer_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    /// Parse instruction automatically detects which instruction is being called
    /// based on the discriminator and returns the instruction and the data
    let (ix, data) = parse_instruction(&pda_rent_payer_api::ID, program_id, data)?;

    match ix {
        PdaRentPayerInstruction::InitializeRentVault => process_initialize_vault(accounts, data)?,
        PdaRentPayerInstruction::CreateNewAccount => process_create_account(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
