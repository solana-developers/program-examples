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
    if fee > 1000 {
        return Err(TutorialError::InvalidFee.into());
    }

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
