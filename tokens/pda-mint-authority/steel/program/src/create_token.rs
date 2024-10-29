use solana_program::system_instruction;
use steel::*;
use api::prelude::*;
use spl_token::state::Mint;


use spl_token::solana_program::program_pack::Pack;
use sysvar::rent::Rent;


pub fn process_create_token(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Start the process
    solana_program::msg!("Starting process_create_token");
    
    // Log the incoming data
    solana_program::msg!("Instruction data length: {}", data.len());

    // Deserialize the instruction data containing token details with error logging
    let args = CreateToken::try_from_bytes(data).map_err(|e| {
        solana_program::msg!("Failed to deserialize instruction data: {:?}", e);
        ProgramError::InvalidInstructionData
    })?;

    solana_program::msg!("Successfully deserialized instruction data");

    // Log account info before unpacking
    solana_program::msg!("Number of accounts provided: {}", accounts.len());
    for (i, acc) in accounts.iter().enumerate() {
        solana_program::msg!("Account #{}: key={}, owner={}, is_signer={}, is_writable={}", 
            i, acc.key, acc.owner, acc.is_signer, acc.is_writable);
    }

    // Extract all the accounts we need from the accounts array using pattern matching
    let [
        payer,              // Account that will pay for the transaction
        mint_account,       // The new token mint account we'll create
        mint_authority,     // PDA that will have authority over the mint
        metadata_account,   // Account that will store token metadata
        token_program,      // SPL Token program
        token_metadata_program,  // Metaplex Token Metadata program
        system_program,     // System program for creating accounts
        rent,              // Rent sysvar
    ] = accounts else {
        solana_program::msg!("Failed to unpack accounts");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    solana_program::msg!("Successfully unpacked accounts");

    // Log that we're starting account validation
    solana_program::msg!("Step 1: Validating accounts...");

    // Make sure the payer signed the transaction
    solana_program::msg!("Checking if payer is a signer");
    if !payer.is_signer {
        solana_program::msg!("Payer must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    solana_program::msg!("Payer is successfully a signer");

    // Verify the mint account is empty (REMOVE the additional validations that were here)
    solana_program::msg!("Checking if mint account is empty");
    if !mint_account.data_is_empty() {
        solana_program::msg!("Mint account is already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    solana_program::msg!("Mint account is empty");

    // Make sure system program can create account
    solana_program::msg!("Checking if system program is executable");
    if !system_program.executable {
        solana_program::msg!("Program is not executable");
        return Err(ProgramError::InvalidAccountData);
    }

    solana_program::msg!("Program is successfully executable");
    
    // Make sure mint account is writable
    solana_program::msg!("Checking if mint is writable");
    if !mint_account.is_writable {
        solana_program::msg!("Mint is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    solana_program::msg!("Mint is successfully writable");

    
    // Make sure metadata account is writable
    solana_program::msg!("Checking if metadata is writable");
    if !metadata_account.is_writable {
        solana_program::msg!("Metadata is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    solana_program::msg!("Metadata is successfully writable");


    // Verify all program IDs match what we expect
    solana_program::msg!("Verifying token program ID");
    token_program.is_program(&spl_token::ID)?;
    solana_program::msg!("Token Program ID successfully verified");

    solana_program::msg!("Verifying metadata program ID");
    token_metadata_program.is_program(&mpl_token_metadata::ID)?;
    solana_program::msg!("Token Metadata Program ID successfully verified");

    solana_program::msg!("Verifying system program ID");
    system_program.is_program(&system_program::ID)?;
    solana_program::msg!("System Program ID successfully verified");

    solana_program::msg!("Verifying rent sysvar...");
    if *rent.key != solana_program::sysvar::rent::ID {
        solana_program::msg!("Invalid rent sysvar");
        return Err(ProgramError::InvalidAccountData);
    }
    solana_program::msg!("Rent sysvar successfully verified");

    solana_program::msg!("Step 2: Verifying PDA...");

    // Derive our PDA and make sure it matches what was passed in
    let (mint_pda, bump) = Pubkey::find_program_address(
        &[MintAuthorityPda::SEED_PREFIX.as_bytes()], 
        &api::ID
    );

    solana_program::msg!("Expected mint PDA: {}", mint_pda);
    solana_program::msg!("Provided mint account: {}", mint_account.key);

    // Verify both mint account and mint authority match our derived PDA
    if mint_account.key != &mint_pda || mint_authority.key != &mint_pda {
        solana_program::msg!("Invalid mint PDA");
        return Err(ProgramError::InvalidSeeds);
    }

    solana_program::msg!("Mint PDA successfully verified");
    

    // Also, verify the metadata PDA derivation is using correct seeds and bump
    let (metadata_pda, _metadata_bump) = Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint_account.key.as_ref()
        ],
        &mpl_token_metadata::ID
    );
    
    if metadata_account.key != &metadata_pda {
        solana_program::msg!("Invalid metadata PDA");
        return Err(ProgramError::InvalidSeeds);
    }

    solana_program::msg!("Metadata PDA successfully verified");

    solana_program::msg!("Step 3: Creating mint account...");
    
    // Calculate space and rent for the mint account
    let mint_space = Mint::LEN;
    let mint_rent = Rent::get()?.minimum_balance(mint_space);

    // Create the mint account using the system program
    solana_program::program::invoke_signed(
        &system_instruction::create_account(
            payer.key,           // From this account
            mint_account.key,    // Create this account
            mint_rent,           // With this much rent
            mint_space as u64,   // And this much space
            &spl_token::id(),    // Owned by token program
        ),
        &[
            payer.clone(),
            mint_account.clone(),
        ],
        &[&[MintAuthorityPda::SEED_PREFIX.as_bytes(), &[bump]]],  // PDA signs
    )?;

    solana_program::msg!("Step 4: Initializing mint...");
    
    // Debug mint account state before initialization
    solana_program::msg!("Mint account state before initialization:");
    solana_program::msg!("  Key: {}", mint_account.key);
    solana_program::msg!("  Owner: {}", mint_account.owner);
    solana_program::msg!("  Data len: {}", mint_account.data_len());
    solana_program::msg!("  Lamports: {}", mint_account.lamports());

    // Debug initialization parameters
    solana_program::msg!("Initializing mint with parameters:");
    solana_program::msg!("  Token Program: {}", spl_token::id());
    solana_program::msg!("  Mint Account: {}", mint_account.key);
    solana_program::msg!("  Mint Authority: {}", mint_authority.key);
    solana_program::msg!("  Decimals: {}", 9);

    let init_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        mint_account.key,
        mint_authority.key,
        Some(mint_authority.key),
        9,
    )?;

    solana_program::msg!("Created initialize_mint instruction");

    // Debug the instruction data
    solana_program::msg!("Initialize mint instruction data: {:?}", init_mint_ix.data);
    solana_program::msg!("Invoking Token Program to initialize mint...");
    let result = solana_program::program::invoke(
        &init_mint_ix,
        &[
            mint_account.clone(),
            rent.clone(),
        ],
    );

    match result {
        Ok(_) => solana_program::msg!("Token Program invocation successful"),
        Err(e) => {
            solana_program::msg!("Token Program invocation failed: {:?}", e);
            return Err(e);
        }
    }

    // Verify initialization immediately
    let mint_data = Mint::unpack(&mint_account.try_borrow_data()?)?;
    if !mint_data.is_initialized {
        solana_program::msg!("Mint account failed to initialize");
        return Err(ProgramError::UninitializedAccount);
    }
    solana_program::msg!("Mint state after initialization:");
    solana_program::msg!("  Is initialized: {}", mint_data.is_initialized);
    solana_program::msg!("  Mint authority: {:?}", mint_data.mint_authority);
    solana_program::msg!("  Supply: {}", mint_data.supply);
    solana_program::msg!("  Decimals: {}", mint_data.decimals);

    solana_program::msg!("Mint initialized successfully");

    solana_program::msg!("Step 5: Creating metadata...");
    
    // Clean up our token's metadata strings
    let token_name = std::str::from_utf8(&args.token_name)
        .map_err(|_| ProgramError::InvalidInstructionData)?
        .trim_matches(char::from(0));
    let token_symbol = std::str::from_utf8(&args.token_symbol)
        .map_err(|_| ProgramError::InvalidInstructionData)?
        .trim_matches(char::from(0));
    let token_uri = std::str::from_utf8(&args.token_uri)
        .map_err(|_| ProgramError::InvalidInstructionData)?
        .trim_matches(char::from(0));

    // Prepare the metadata for our token
    let data = mpl_token_metadata::types::DataV2 {
        name: token_name.to_string(),
        symbol: token_symbol.to_string(),
        uri: token_uri.to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    // Create the metadata instruction
    let create_metadata_account_v3 = mpl_token_metadata::instructions::CreateMetadataAccountV3 {
        metadata: *metadata_account.key,
        mint: *mint_account.key,
        mint_authority: *mint_authority.key,
        payer: *payer.key,
        update_authority: (*mint_authority.key, true),  // Make authority mutable
        system_program: *system_program.key,
        rent: Some(*rent.key),
    };

    // Build the complete metadata instruction with our data
    let ix = create_metadata_account_v3.instruction(
        mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs {
            data,
            is_mutable: true,
            collection_details: None,
        }
    );

    solana_program::msg!("Step 6: Creating metadata account...");
    
    // Create the metadata account, signed by our PDA
    solana_program::program::invoke_signed(
        &ix,
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            mint_authority.clone(),
            system_program.clone(),
            rent.clone(),
        ],
        &[&[MintAuthorityPda::SEED_PREFIX.as_bytes(), &[bump]]],
    )?;

    solana_program::msg!("Token creation complete!");
    Ok(())
}