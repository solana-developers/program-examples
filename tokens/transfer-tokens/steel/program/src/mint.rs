use solana_program::msg;
use steel::*;
use transfer_tokens_api::prelude::*;

pub fn process_mint(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // parse args.
    let args = Mint::try_from_bytes(data)?;
    let quantity = u64::from_le_bytes(args.quantity);

    // Load accounts.
    let [mint_authority_info, recipient_info, mint_info, associated_token_account_info, token_program, associated_token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    mint_authority_info.is_signer()?;
    mint_info.as_mint()?;
    token_program.is_program(&spl_token::ID)?;

    if associated_token_account_info.lamports() == 0 {
        msg!("Creating associated token account...");
        create_associated_token_account(
            mint_authority_info,
            recipient_info,
            associated_token_account_info,
            mint_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    } else {
        msg!("Associated token account exists.");
    }
    msg!(
        "Associated Token Address: {}",
        associated_token_account_info.key
    );

    msg!("Minting {} tokens to associated token account...", quantity);

    solana_program::program::invoke(
        &spl_token::instruction::mint_to(
            &spl_token::id(),
            mint_info.key,
            associated_token_account_info.key,
            mint_authority_info.key,
            &[mint_authority_info.key],
            quantity,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            associated_token_account_info.clone(),
            mint_authority_info.clone(),
        ],
    )?;

    msg!("Token minted successfully.");

    Ok(())
}
