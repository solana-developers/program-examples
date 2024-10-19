use steel::*;
use steel_api::prelude::*;
use solana_program::system_instruction::transfer;

pub fn process_transfer_with_cpi(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = TransferWithCPI::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [payer, recipient, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // validate signer

    payer.is_signer()?;

    payer.send(amount, recipient);

    system_program.is_program(&system_program::ID)?;


    // transfer(payer.key, recipient.key, amount);


    Ok(())
}
