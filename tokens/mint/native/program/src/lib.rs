use {
    borsh::{
        BorshSerialize, BorshDeserialize,
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo}, 
        entrypoint, 
        entrypoint::ProgramResult, 
        msg, 
        program::invoke,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
        sysvar::Sysvar,
    },
    spl_token::{
        instruction as token_instruction,
    },
    mpl_token_metadata::{
        instruction as mpl_instruction,
    },
};


entrypoint!(process_instruction);


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    const MINT_SIZE: u64 = 82;

    let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    let metadata_account = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let token_metadata_program = next_account_info(accounts_iter)?;

    let token_metadata = TokenMetadata::try_from_slice(instruction_data)?;
    
    msg!("Creating mint account...");
    msg!("Mint: {}", mint_account.key);
    invoke(
        &system_instruction::create_account(
            &mint_authority.key,
            &mint_account.key,
            (Rent::get()?).minimum_balance(MINT_SIZE as usize),
            MINT_SIZE,
            &token_program.key,
        ),
        &[
            mint_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
        ]
    )?;

    msg!("Initializing mint account...");
    msg!("Mint: {}", mint_account.key);
    invoke(
        &token_instruction::initialize_mint(
            &token_program.key,
            &mint_account.key,
            &mint_authority.key,
            Some(&mint_authority.key),
            9,                              // 9 Decimals
        )?,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
            rent.clone(),
        ]
    )?;

    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", metadata_account.key);
    invoke(
        &mpl_instruction::create_metadata_accounts_v2(
            *token_metadata_program.key,    // Program ID (the Token Metadata Program)
            *metadata_account.key,          // Metadata Account
            *mint_account.key,              // Mint Account
            *mint_authority.key,            // Mint Authority
            *mint_authority.key,            // Payer
            *mint_authority.key,            // Update Authority
            token_metadata.title,           // Name
            token_metadata.symbol,          // Symbol
            token_metadata.uri,             // URI
            None,                           // Creators
            0,                              // Seller fee basis points
            true,                           // Update authority is signer
            false,                          // Is mutable
            None,                           // Collection
            None,                           // Uses
        ),
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            token_metadata_program.clone(),
            rent.clone(),
        ],
    )?;

    msg!("Token mint process completed successfully.");

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenMetadata {
    title: String,
    symbol: String,
    uri: String,
}