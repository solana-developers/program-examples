use steel::*;
use token_swap_api::prelude::*;

pub fn process_create_amm(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [payer_info, admin_info, amm_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let args = CreateAmm::try_from_bytes(data)?;
    let id = args.id;
    let fee = args.fee;

    // Check fee is valid.
    if u16::from_le_bytes(fee) > 10000 {
        return Err(TokenSwapError::InvalidFee.into());
    }

    // check payer is signer of the transaction
    payer_info.is_signer()?;
    amm_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[id.as_ref()], &token_swap_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Initialize amm account.
    create_account::<Amm>(
        amm_info,
        system_program,
        payer_info,
        &token_swap_api::ID,
        &[id.as_ref()],
    )?;
    let amm = amm_info.as_account_mut::<Amm>(&token_swap_api::ID)?;
    amm.admin = *admin_info.key;
    amm.id = id;
    amm.fee = fee;
    Ok(())
}
