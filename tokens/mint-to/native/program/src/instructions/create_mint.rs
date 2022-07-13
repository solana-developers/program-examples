use {
    solana_program::{
        account_info::{next_account_info, AccountInfo}, 
        entrypoint::ProgramResult, 
        msg, 
        program::invoke,
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


pub fn create_mint(
    accounts: &[AccountInfo],
    token_title: String,
    token_symbol: String,
    token_uri: String,
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
            token_title,
            token_symbol,
            token_uri,
            None,
            0,
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

    msg!("Token mint created successfully.");

    Ok(())
}

