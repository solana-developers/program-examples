use steel::*;
use steel_api::prelude::*;
use solana_program::msg;

pub fn process_transfer_with_program(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = TransferWithProgram::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [payer, recipient] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // validate signer

    msg!("{}",payer.key);
    
    msg!("{}",recipient.key);

    payer.is_signer()?;
    payer.send(amount, recipient);

    Ok(())
}
