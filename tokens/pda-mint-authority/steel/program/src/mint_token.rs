use steel::*;
use api::instruction::MintToken;
use api::prelude::*;
use spl_token::instruction::mint_to;
use spl_associated_token_account::*;

pub fn process_mint_token(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {

    let _args = MintToken::try_from_bytes(data)?;

    let [
    payer,
    mint_account,
    associated_token_account,
    token_program,
    associated_token_program,
    system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let (_mint_pda, bump) = Pubkey::find_program_address(&[MintAuthorityPda::SEED_PREFIX.as_bytes()], &api::ID);

    // Validate accounts
    payer.is_signer()?;

    mint_account
        .is_writable()?
        .has_seeds(&[MintAuthorityPda::SEED_PREFIX.as_bytes()], bump, &api::ID)?
        .has_owner(payer.key)?;


    associated_token_account
        .to_token_account()?
        .check(|m| m.mint == *mint_account.key)?
        .check(|m| m.owner == *payer.key)?;

    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;
    system_program.is_program(&system_program::ID)?;


    // Create associated token account if it doesn't exist
    if associated_token_account.data_is_empty() {
        solana_program::msg!("Creating associated token account...");
        solana_program::program::invoke(
            &spl_associated_token_account::instruction::create_associated_token_account(
                payer.key,
                payer.key,
                mint_account.key,
                token_program.key,
            ),
            &[
                mint_account.clone(),
                associated_token_account.clone(),
                payer.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
        )?;
    } else {
        solana_program::msg!("Associated token account exists.");
    }

    // Mint tokens
    // let mint_authority_seeds = &[b"mint", &[bump]];
    // mint_to(
    //     mint_account.key(),
    //     associated_token_account,
    //     mint_account,
    //     token_program,
    //     args.amount,
    //     &[&mint_authority_seeds],
    // )?;

    solana_program::msg!("Minting Token to associated token account...");
    solana_program::program::invoke_signed(
        &mint_to(
            token_program.key,
            mint_account.key,
            associated_token_account.key,
            payer.key,
            &[payer.key],
            1,
        )?,
        &[
            mint_account.clone(),
            payer.clone(),
            associated_token_account.clone(),
            token_program.clone(),
        ],
        &[&[MintAuthorityPda::SEED_PREFIX.as_bytes(), &[bump]]],
    )?;

    Ok(())
}