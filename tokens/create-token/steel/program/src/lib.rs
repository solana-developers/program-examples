use {
    borsh::{BorshDeserialize, BorshSerialize},
    mpl_token_metadata::{instructions as mpl_instruction, types::DataV2},
    solana_program::{msg, program::invoke, program_pack::Pack, rent::Rent, system_instruction},
    spl_token::state::Mint,
    steel::*,
};

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateTokenArgs {
    pub token_title: String,
    pub token_symbol: String,
    pub token_uri: String,
    pub token_decimals: u8,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let args = CreateTokenArgs::try_from_slice(instruction_data)?;

    let [mint_account, mint_authority, metadata_account, payer, rent, system_program, token_program, _token_metadata_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // First create the account for the Mint
    //
    msg!("Creating mint account...");
    msg!("Mint: {}", mint_account.key);
    invoke(
        &system_instruction::create_account(
            payer.key,
            mint_account.key,
            (Rent::get()?).minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            token_program.key,
        ),
        &[
            mint_account.clone(),
            payer.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )?;

    // Now initialize that account as a Mint (standard Mint)
    //
    msg!("Initializing mint account...");
    msg!("Mint: {}", mint_account.key);

    initialize_mint(
        mint_account,
        mint_authority,
        Some(mint_authority),
        token_program,
        rent,
        args.token_decimals,
    )?;

    // Now create the account for that Mint's metadata
    //
    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", metadata_account.key);

    let ix = &mpl_instruction::CreateMetadataAccountV3 {
        metadata: *metadata_account.key,
        mint: *mint_account.key,
        mint_authority: *mint_authority.key,
        payer: *payer.key,
        rent: None,
        system_program: *system_program.key,
        update_authority: (*payer.key, true),
    }
    .instruction(mpl_instruction::CreateMetadataAccountV3InstructionArgs {
        data: DataV2 {
            name: args.token_title,
            symbol: args.token_symbol,
            uri: args.token_uri,
            creators: None,
            seller_fee_basis_points: 0,
            collection: None,
            uses: None,
        },
        collection_details: None,
        is_mutable: false,
    });

    invoke(
        ix,
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            payer.clone(),
            system_program.clone(),
            rent.clone(),
        ],
    )?;

    msg!("Token mint created successfully.");

    Ok(())
}
