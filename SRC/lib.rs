use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke_signed},
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use solana_program::ed25519_program::{ID as ED25519_PROGRAM_ID};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Expected Accounts:
    let signer_account = next_account_info(accounts_iter)?;  // The owner who signs the transaction
    let custodied_account = next_account_info(accounts_iter)?;  // Account holding funds
    let recipient_account = next_account_info(accounts_iter)?;  // Recipient of funds
    let system_program = next_account_info(accounts_iter)?;  // System program account

    // Verify that the signer has provided a valid Ed25519 signature
    let signature_data = instruction_data; // Assuming signature data is provided in the instruction data
    msg!("Verifying Ed25519 signature...");

    solana_program::program::invoke(
        &solana_program::ed25519_instruction::new_ed25519_instruction(
            signer_account.key,
            signature_data,
        ),
        &[signer_account.clone()],
    )?;

    msg!("Signature verified successfully!");

    // Perform fund transfer after verification
    let transfer_lamports = custodied_account.lamports();
    msg!("Transferring {} lamports from custodied account to recipient", transfer_lamports);

    invoke_signed(
        &system_instruction::transfer(
            custodied_account.key,
            recipient_account.key,
            transfer_lamports,
        ),
        &[custodied_account.clone(), recipient_account.clone(), system_program.clone()],
        &[],
    )?;

    msg!("Funds transferred successfully!");

    Ok(())
}
