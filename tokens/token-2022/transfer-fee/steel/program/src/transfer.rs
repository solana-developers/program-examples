use solana_program::{msg, program::invoke};
use spl_token_2022::{
    extension::{
        transfer_fee::{instruction::transfer_checked_with_fee, TransferFeeConfig},
        BaseStateWithExtensions, StateWithExtensions,
    },
    state::Mint,
};
use steel::*;
use steel_api::prelude::*;

// transfer fees are automatically deducted from the transfer amount
// recipients receives (transfer amount - fees)
// transfer fees are stored directly on the recipient token account and must be "harvested"
pub fn process_transfer(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    let args = Transfer::try_from_bytes(&_data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, mint_info, source_token_account, destination_token_account, system_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    source_token_account.is_writable()?;
    destination_token_account.is_writable()?;
    token_program.is_program(&spl_token_2022::ID)?;
    system_program.is_program(&system_program::ID)?;

    //Get mint extension info
    let mint_data = mint_info.data.borrow();
    let mint_with_extension = StateWithExtensions::<Mint>::unpack(&mint_data)?;
    msg!(&mint_with_extension.base.decimals.to_string());

    //calculate expected fee and epoch
    let epoch = Clock::get()?.epoch;
    let extension_data = mint_with_extension.get_extension::<TransferFeeConfig>()?;
    let fee = extension_data.calculate_epoch_fee(epoch, amount).unwrap();

    invoke(
        &transfer_checked_with_fee(
            token_program.key,
            source_token_account.key,
            mint_info.key,
            destination_token_account.key,
            signer_info.key,
            &[],
            amount,
            mint_with_extension.base.decimals,
            fee,
        )?,
        &[
            source_token_account.clone(),
            mint_info.clone(),
            destination_token_account.clone(),
            signer_info.clone(),
        ],
    )?;

    msg!("Transfer Fee Extension: Transfer Successful.");
    Ok(())
}
