use {
    solana_program::{
        account_info::{next_account_info, AccountInfo}, 
        entrypoint::ProgramResult, 
        msg, 
        program::invoke_signed,
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
        sysvar::Sysvar,
    },
    spl_token::{
        instruction as token_instruction,
        state::Mint,
    },
    mpl_token_metadata::{
        instruction as mpl_instruction,
    },
};


pub fn create_token_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_title: String,
    token_symbol: String,
    token_uri: String,
    mint_authority_pda_bump: u8,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let metadata_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let token_metadata_program = next_account_info(accounts_iter)?;

    msg!("Creating mint authority...");
    msg!("Mint Authority: {}", mint_authority.key);
    invoke_signed(
        &system_instruction::create_account(
            &payer.key,
            &mint_authority.key,
            (Rent::get()?).minimum_balance(8) as u64,
            8,
            &program_id,
        ),
        &[
            payer.clone(),
            mint_authority.clone(),
            system_program.clone(),
        ],
        &[&[
            b"mint_authority_", 
            mint_account.key.as_ref(),
            &[mint_authority_pda_bump],
        ]]
    )?;
    
    msg!("Creating mint account...");
    msg!("Mint: {}", mint_account.key);
    invoke_signed(
        &system_instruction::create_account(
            &payer.key,
            &mint_account.key,
            (Rent::get()?).minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            &token_program.key,
        ),
        &[
            mint_account.clone(),
            payer.clone(),
            token_program.clone(),
        ],
        &[&[
            b"mint_authority_", 
            mint_account.key.as_ref(),
            &[mint_authority_pda_bump],
        ]]
    )?;

    msg!("Initializing mint account...");
    msg!("Mint: {}", mint_account.key);
    invoke_signed(
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
        ],
        &[&[
            b"mint_authority_", 
            mint_account.key.as_ref(),
            &[mint_authority_pda_bump],
        ]]
    )?;

    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", metadata_account.key);
    invoke_signed(
        &mpl_instruction::create_metadata_accounts_v2(
            *token_metadata_program.key,
            *metadata_account.key,
            *mint_account.key,
            *mint_authority.key,
            *payer.key,
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
            payer.clone(),
            token_metadata_program.clone(),
            rent.clone(),
        ],
        &[&[
            b"mint_authority_", 
            mint_account.key.as_ref(),
            &[mint_authority_pda_bump],
        ]]
    )?;

    msg!("Token mint created successfully.");

    Ok(())
}

pub struct MintAuthority {}