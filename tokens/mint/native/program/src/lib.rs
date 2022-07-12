use {
    borsh::{
        BorshSerialize, BorshDeserialize,
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo}, 
        entrypoint, 
        entrypoint::ProgramResult, 
        msg, 
        native_token::LAMPORTS_PER_SOL,
        program::invoke,
        pubkey::Pubkey,
        system_instruction,
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
            LAMPORTS_PER_SOL,
            82,
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
            9,
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
            *token_metadata_program.key, 
            *metadata_account.key, 
            *mint_account.key, 
            *mint_authority.key, 
            *mint_authority.key, 
            *mint_authority.key, 
            token_metadata.title,
            token_metadata.symbol,
            token_metadata.uri,
            None,
            1,
            true, 
            false, 
            None, 
            None,
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