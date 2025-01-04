use pda_mint_authority_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_mint(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // parse args.
    let args = Mint::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [payer_info, mint_info, ata_info, mint_authority_info, token_program, associated_token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    msg!("Minting tokens to associated token account...");
    msg!("Mint: {:?}", mint_info);
    msg!("Token Address: {:?}", &ata_info);

    // validation
    payer_info.is_signer()?;
    mint_info.as_mint()?;
    token_program.is_program(&spl_token::ID)?;

    if ata_info.lamports() == 0 {
        msg!("Creating associated token account...");
        create_associated_token_account(
            payer_info,
            payer_info,
            ata_info,
            mint_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
        msg!("Associated Token Address: {}", ata_info.key);
    } else {
        msg!("Associated token account exists.");
    }

    mint_authority_info
        .is_writable()?
        .has_seeds(&[MINT_AUTHORITY], &pda_mint_authority_api::ID)?;
    ata_info
        .is_writable()?
        .as_associated_token_account(payer_info.key, mint_info.key)?;

    msg!("Minting token to associated token account...");
    msg!("Mint: {}", mint_info.key);
    msg!("Token Address: {}", ata_info.key);

    solana_program::program::invoke_signed(
        &spl_token::instruction::mint_to(
            &spl_token::id(),
            mint_info.key,
            ata_info.key,
            mint_authority_info.key,
            &[mint_authority_info.key],
            amount,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            ata_info.clone(),
            mint_authority_info.clone(),
        ],
        &[&[MINT_AUTHORITY, &[mint_authority_pda().1]]],
    )?;

    msg!("Token minted successfully.");

    Ok(())
}
