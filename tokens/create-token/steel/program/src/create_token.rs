use steel_api::prelude::*;
use spl_token::state::Mint;
use solana_program::msg;
use solana_program::program_pack::Pack;
use steel::*;

pub fn process_create_token(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {

    //Parse Args
    let args = CreateToken::try_from_bytes(data)?;
    let name = String::from_utf8(args.data.name.to_vec()).expect("Invalid UTF-8");
    let symbol = String::from_utf8(args.data.symbol.to_vec()).expect("Invalid UTF-8");
    let uri = String::from_utf8(args.data.uri.to_vec()).expect("Invalid UTF-8");
    let decimals = args.data.decimals;
    msg!("Parsed Arguments");
    
    // Load accounts.
    let [signer_info, mint_info,metadata_info, system_program, token_program, metadata_program, rent_sysvar] = 
    accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);        
    };
    msg!("Loaded Accounts");

    //Validation
    signer_info.is_signer()?;
    mint_info.is_signer()?.is_empty()?.is_writable()?;
    metadata_info.is_empty()?.is_writable()?
    .has_seeds(
        &[METADATA, mpl_token_metadata::ID.as_ref(), mint_info.key.as_ref()],
        &mpl_token_metadata::ID,
    )?
    ;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    metadata_program.is_program(&mpl_token_metadata::ID)?;
    rent_sysvar.is_sysvar(&sysvar::rent::ID)?;
    msg!("Accounts validated");

    // First, create an account for the Mint.
    //
    msg!("Creating mint account...");
    msg!("Mint: {}", mint_info.key);
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            signer_info.key,
            mint_info.key,
            (solana_program::rent::Rent::get()?).minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            token_program.key,
        ),
        &[
            mint_info.clone(),
            signer_info.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )?;

    // Second, Initialize account as Mint Account
    //
    msg!("Initializing mint account...");
    solana_program::program::invoke(
        &spl_token::instruction::initialize_mint(
            token_program.key,
            mint_info.key,
            signer_info.key,
            Some(signer_info.key),
            decimals, // 9 Decimals for the default SPL Token standard
        )?,
        &[
            mint_info.clone(),
            signer_info.clone(),
            token_program.clone(),
            rent_sysvar.clone(),
        ],
    )?;


    // Lastly, create the account for that Mint's metadata
    //
    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", metadata_info.key);
    mpl_token_metadata::instructions::CreateMetadataAccountV3Cpi {
        __program: metadata_program,
        metadata: metadata_info,
        mint: mint_info,
        mint_authority: signer_info,
        payer: signer_info,
        update_authority: (signer_info, true),
        system_program,
        rent: Some(rent_sysvar),
        __args: mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs {
            data: mpl_token_metadata::types::DataV2 {
                name: name,
                symbol: symbol,
                uri: uri,
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
