use solana_program::msg;
use solana_program::program_pack::Pack;
use spl_token::state::Mint;
use steel::*;
use steel_api::prelude::*;

pub fn process_create(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // parse args.
    let args = Create::try_from_bytes(data)?;
    let token_name = String::from_utf8(args.token_name.to_vec()).expect("Invalid UTF-8");
    let token_symbol = String::from_utf8(args.token_symbol.to_vec()).expect("Invalid UTF-8");
    let token_uri = String::from_utf8(args.token_uri.to_vec()).expect("Invalid UTF-8");

    // Load accounts.
    let [payer_info, mint_info, mint_authority_info, metadata_info, token_program, system_program, rent_sysvar, token_metadata_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // validation
    payer_info.is_signer()?;
    mint_info.to_mint()?;
    token_program.is_program(&spl_token::ID)?;
    rent_sysvar.is_sysvar(&sysvar::rent::ID)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    token_metadata_program.is_program(&mpl_token_metadata::ID)?;
    rent_sysvar.is_sysvar(&sysvar::rent::ID)?;

    mint_authority_info.is_empty()?.is_writable()?.has_seeds(
        &[MINT_AUTHORITY],
        args.mint_authority_bump,
        &steel_api::ID,
    )?;

    // Initialize mint authority.
    create_account::<MintAuthorityPda>(
        mint_authority_info,
        &steel_api::ID,
        &[MINT_AUTHORITY, &[args.mint_authority_bump]],
        system_program,
        payer_info,
    )?;

    // First create the account for the Mint
    //
    msg!("Creating mint account...");
    msg!("Mint: {}", mint_info.key);
    allocate_account(
        mint_info,
        &spl_token::ID,
        Mint::LEN,
        &[MINT, MINT_NOISE.as_slice(), &[args.mint_bump]],
        system_program,
        payer_info,
    )?;

    // Now initialize that account as a Mint (standard Mint)
    //
    msg!("Initializing mint account...");
    msg!("Mint: {}", mint_info.key);
    solana_program::program::invoke(
        &spl_token::instruction::initialize_mint(
            &spl_token::ID,
            mint_info.key,
            mint_authority_info.key,
            Some(mint_authority_info.key),
            9, // 9 Decimals for the default SPL Token standard,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            mint_authority_info.clone(),
            rent_sysvar.clone(),
        ],
    )?;

    // Now create the account for that Mint's metadata
    //
    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", metadata_info.key);
    mpl_token_metadata::instructions::CreateMetadataAccountV3Cpi {
        __program: token_metadata_program,
        metadata: metadata_info,
        mint: mint_info,
        mint_authority: mint_authority_info,
        payer: payer_info,
        update_authority: (mint_authority_info, true),
        system_program,
        rent: Some(rent_sysvar),
        __args: mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs {
            data: mpl_token_metadata::types::DataV2 {
                name: token_name,
                symbol: token_symbol,
                uri: token_uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            is_mutable: true,
            collection_details: None,
        },
    }
    .invoke_signed(&[&[MINT_AUTHORITY, &[args.mint_authority_bump]]])?;

    msg!("Token mint created successfully.");

    Ok(())
}
