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
    spl_associated_token_account::{
        instruction as token_account_instruction,
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

    let metadata_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;
    let token_metadata_program = next_account_info(accounts_iter)?;

    let nft_metadata = NftMetadata::try_from_slice(instruction_data)?;
    
    msg!("Creating mint account...");
    msg!("Mint: {}", mint_account.key);
    invoke(
        &system_instruction::create_account(
            &mint_authority.key,
            &mint_account.key,
            (Rent::get()?).minimum_balance(Mint::LEN),
            Mint::LEN as u64,
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
            nft_metadata.title,           // Name
            nft_metadata.symbol,          // Symbol
            nft_metadata.uri,             // URI
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

    msg!("NFT mint created successfully.");

    msg!("Creating token account...");
    msg!("Token Address: {}", token_account.key);    
    invoke(
        &token_account_instruction::create_associated_token_account(
            &mint_authority.key,
            &mint_authority.key,
            &mint_account.key,
        ),
        &[
            mint_account.clone(),
            token_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
            associated_token_program.clone(),
        ]
    )?;

    msg!("Minting NFT to token account...");
    msg!("NFT Mint: {}", mint_account.key);   
    msg!("Token Address: {}", token_account.key);
    invoke(
        &token_instruction::mint_to(
            &token_program.key,
            &mint_account.key,
            &token_account.key,
            &mint_authority.key,
            &[&mint_authority.key],
            1,
        )?,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            token_account.clone(),
            token_program.clone(),
            rent.clone(),
        ]
    )?;

    msg!("Disabling future minting...");
    msg!("NFT Mint: {}", mint_account.key);   
    invoke(
        &token_instruction::set_authority(
            &token_program.key, 
            &mint_account.key, 
            None, 
            token_instruction::AuthorityType::MintTokens, 
            &mint_authority.key, 
            &[&mint_authority.key]
        )?,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("NFT minting disabled successfully.");
    msg!("Token mint process completed successfully.");

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NftMetadata {
    title: String,
    symbol: String,
    uri: String,
}