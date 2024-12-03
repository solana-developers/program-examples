use steel::*;
use token_swap_api::prelude::*;

pub fn process_create_amm(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = CreateAmm::try_from_bytes(data)?;
    let fee = u16::from_le_bytes(args.fee);
    let id = Pubkey::new_from_array(args.id);

    // Load accounts.
    let [signer_info, amm_info, admin_infos, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // validating accounts and input
    signer_info.is_signer()?;
    amm_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[id.as_ref()], &token_swap_api::ID)?;
    assert(
        admin_infos.is_writable == false,
        TutorialError::ValidationBreached,
        "Admin account should be read only",
    )?;
    assert(
        fee < 1000,
        TutorialError::InvalidFee,
        &TutorialError::InvalidFee.to_string(),
    )?;
    system_program.is_program(&system_program::ID)?;

    // - Program Logic - create Amm account and populate fields
    create_account::<Amm>(
        amm_info,
        system_program,
        signer_info,
        &token_swap_api::ID,
        &[id.as_ref()],
    )?;

    let amm: &mut Amm = amm_info.as_account_mut::<Amm>(&token_swap_api::ID)?;

    amm.id = id;
    amm.admin = *admin_infos.key;
    amm.fee = fee;

    Ok(())
}
