use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult, 
        msg, 
        program::{invoke, invoke_signed},
        pubkey::Pubkey,
    },
    spl_token::{
        instruction as token_instruction,
    },
    spl_associated_token_account::{
        instruction as token_account_instruction,
    },
};


pub fn mint_to_wallet(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    mint_authority_pda_bump: u8,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

    msg!("Creating token account...");
    msg!("Token Address: {}", token_account.key);    
    invoke(
        &token_account_instruction::create_associated_token_account(
            &payer.key,
            &payer.key,
            &mint_account.key,
        ),
        &[
            mint_account.clone(),
            token_account.clone(),
            payer.clone(),
            token_program.clone(),
            associated_token_program.clone(),
        ]
    )?;

    msg!("Minting {} tokens to token account...", amount);
    msg!("Mint: {}", mint_account.key);   
    msg!("Token Address: {}", token_account.key);
    invoke_signed(
        &token_instruction::mint_to(
            &token_program.key,
            &mint_account.key,
            &token_account.key,
            &mint_authority.key,
            &[&mint_authority.key],
            amount,
        )?,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            token_account.clone(),
            token_program.clone(),
            rent.clone(),
        ],
        &[&[
            b"mint_authority_", 
            mint_account.key.as_ref(),
            &[mint_authority_pda_bump],
        ]]
    )?;

    msg!("Tokens minted to wallet successfully.");

    Ok(())
}

