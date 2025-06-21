use solana_program::{msg, program::invoke};
use spl_token_2022::extension::transfer_fee::instruction::set_transfer_fee;
use steel::*;
use steel_api::prelude::*;

// Note that there is a 2 epoch delay from when new fee updates take effect
// This is a safely feature built into the extension
// https://github.com/solana-program/token-2022/blob/main/program/src/extension/transfer_fee/processor.rs#L100
pub fn process_update_fee(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    let args = UpdateFee::try_from_bytes(&_data)?;
    let maximum_fee = u64::from_le_bytes(args.maximum_fee);
    let transfer_fee_basis_points = u16::from_le_bytes(args.transfer_fee_basis_points);

    //Load accounts
    let [signer_info, mint_info, system_program, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    mint_info.is_writable()?;
    token_program.is_program(&spl_token_2022::ID)?;
    system_program.is_program(&system_program::ID)?;

    invoke(
        &set_transfer_fee(
            token_program.key,
            mint_info.key,
            signer_info.key,
            &[],
            transfer_fee_basis_points,
            maximum_fee,
        )?,
        &[mint_info.clone(), signer_info.clone()],
    )?;

    msg!("Transfer Fee Extension: Update Transfer Fees Successful");
    Ok(())
}
