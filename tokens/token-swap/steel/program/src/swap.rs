use steel::*;
use token_swap_api::prelude::*;
pub fn process_swap(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // // Load accounts.
    // let [payer_info, amm_info, pool_info, pool_authority_info, mint_liquidity_info, mint_a_info, mint_b_info, pool_account_a_info, pool_account_b_info, token_program, system_program, rent_sysvar] =
    //     accounts
    // else {
    //     return Err(ProgramError::NotEnoughAccountKeys);
    // };

    // // Check payer account is signer.
    // payer_info.is_signer()?;
    // token_program.is_program(&spl_token::ID)?;
    // system_program.is_program(&system_program::ID)?;

    Ok(())
}
