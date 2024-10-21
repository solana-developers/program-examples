use create_token_api::prelude::*;
use solana_program::msg;
use solana_program::program_pack::Pack;
use spl_token::state::Mint;
use steel::*;

pub fn process_create(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // parse args.
    let args = Create::try_from_bytes(data)?;
    let token_name = String::from_utf8(args.token_name.to_vec()).expect("Invalid UTF-8");
    let token_symbol = String::from_utf8(args.token_symbol.to_vec()).expect("Invalid UTF-8");
    let token_uri = String::from_utf8(args.token_uri.to_vec()).expect("Invalid UTF-8");
    let token_decimals = args.decimals;

    // Load accounts.
    let [payer_info, mint_info, metadata_info, system_program, token_program, metadata_program, rent_sysvar] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // validation
    payer_info.is_signer()?;
    mint_info.is_empty()?.is_writable()?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    rent_sysvar.is_sysvar(&sysvar::rent::ID)?;

    // First create the account for the Mint
    //
    msg!("Creating mint account...");
    msg!("Mint: {}", mint_info.key);
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            payer_info.key,
            mint_info.key,
            (solana_program::rent::Rent::get()?).minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            token_program.key,
        ),
        &[
            mint_info.clone(),
            payer_info.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )?;

    // Now initialize that account as a Mint (standard Mint)
    //
    msg!("Initializing mint account...");
    msg!("Mint: {}", mint_info.key);
    solana_program::program::invoke(
        &spl_token::instruction::initialize_mint(
            token_program.key,
            mint_info.key,
            payer_info.key,
            Some(payer_info.key),
            token_decimals, // 9 Decimals for the default SPL Token standard
        )?,
        &[
            mint_info.clone(),
            payer_info.clone(),
            token_program.clone(),
            rent_sysvar.clone(),
        ],
    )?;

    // Now create the account for that Mint's metadata
    //
    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", metadata_info.key);
    mpl_token_metadata::instructions::CreateMetadataAccountV3Cpi {
        __program: metadata_program,
        metadata: metadata_info,
        mint: mint_info,
        mint_authority: payer_info,
        payer: payer_info,
        update_authority: (payer_info, true),
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
    .invoke()?;

    msg!("Token mint created successfully.");

    Ok(())
}
