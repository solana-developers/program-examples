use solana_program::msg;
use steel::*;
use steel_api::prelude::*;

pub fn process_mint(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // parse args.
    let args = Mint::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, mint_info, to_info, authority_info, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    msg!("Minting tokens to associated token account...");
    msg!("Mint: {:?}", mint_info);
    msg!("Token Address: {:?}", &to_info);

    // validation
    signer_info.is_signer()?;
    mint_info.to_mint()?;
    token_program.is_program(&spl_token::ID)?;

    to_info
        .is_writable()?
        .to_associated_token_account(signer_info.key, mint_info.key)?
        .check(|t| t.owner == *signer_info.key)?
        .check(|t| t.mint == *mint_info.key)?;

    token_program.is_program(&spl_token::ID)?;

    solana_program::program::invoke_signed(
        &spl_token::instruction::mint_to(
            &spl_token::id(),
            mint_info.key,
            to_info.key,
            authority_info.key,
            &[authority_info.key],
            amount,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            to_info.clone(),
            authority_info.clone(),
        ],
        &[&[MINT_AUTHORITY, &[MINT_AUTHORITY_BUMP]]],
    )?;

    msg!("Token minted successfully.");

    Ok(())
}
