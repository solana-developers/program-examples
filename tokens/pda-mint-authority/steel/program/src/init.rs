use pda_mint_authority_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_init(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [mint_authority_info, payer_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // validation
    payer_info.is_signer()?;
    mint_authority_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[MINT_AUTHORITY], &pda_mint_authority_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    msg!("Creating mint authority PDA...");
    msg!("Mint Authority: {}", &mint_authority_info.key);
    create_account::<MintAuthorityPda>(
        mint_authority_info,
        system_program,
        payer_info,
        &pda_mint_authority_api::ID,
        &[MINT_AUTHORITY],
    )?;

    let mint_authority =
        mint_authority_info.as_account_mut::<MintAuthorityPda>(&pda_mint_authority_api::ID)?;
    let mint_authority_bump = mint_authority_pda().1;
    mint_authority.bump = mint_authority_bump;

    Ok(())
}
