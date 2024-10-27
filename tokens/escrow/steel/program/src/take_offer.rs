use escrow_api::prelude::*;
use steel::*;

pub fn process_take_offer(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [taker_info, maker_info, token_mint_a_info, token_mint_b_info, taker_token_account_a_info, taker_token_account_b_info, maker_token_account_b_info, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    taker_info.is_signer()?;

    Ok(())
}
