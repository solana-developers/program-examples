use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult, 
        msg, 
        program::invoke,
        pubkey::Pubkey,
    },
    spl_token::{
        instruction as token_instruction,
    },
    spl_associated_token_account::{
        instruction as token_account_instruction,
    },
};


pub fn transfer_to_wallet(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    let owner_token_account = next_account_info(accounts_iter)?;
    let recipient_token_account = next_account_info(accounts_iter)?;
    let owner = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

    msg!("Creating token account for recipient...");
    msg!("Recipient Token Address: {}", recipient_token_account.key); 
    let create_recipient_token_account_ix = &token_account_instruction::create_associated_token_account(
        &recipient.key,
        &recipient.key,
        &mint_account.key,
    );
    let create_recipient_token_account_accts = &[
        mint_account.clone(),
        recipient_token_account.clone(),
        recipient.clone(),
        token_program.clone(),
        associated_token_program.clone(),
    ];
    match invoke(create_recipient_token_account_ix, create_recipient_token_account_accts) {
        Ok(_) => msg!("Recipient token account created successfully."),
        Err(_) => msg!("Recipient token account exists! Using..."),
    }

    msg!("Minting {} tokens to token account...", amount);
    msg!("Mint: {}", mint_account.key);   
    msg!("Owner Token Address: {}", owner_token_account.key);
    msg!("Recipient Token Address: {}", recipient_token_account.key);
    invoke(
        &token_instruction::transfer(
            &token_program.key,
            &owner_token_account.key,
            &recipient_token_account.key,
            &owner.key,
            &[&owner.key, &recipient.key],
            amount,
        )?,
        &[
            mint_account.clone(),
            owner_token_account.clone(),
            recipient_token_account.clone(),
            owner.clone(),
            recipient.clone(),
            token_program.clone(),
        ]
    )?;

    msg!("Tokens transferred to wallet successfully.");

    Ok(())
}

