use solana_program::msg;
use steel::{transfer as transfer_spl_tokens, *};
use transfer_tokens_api::prelude::*;

pub fn process_transfer(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // parse args.
    let args = Transfer::try_from_bytes(data)?;
    let quantity = u64::from_le_bytes(args.quantity);

    // Load accounts.
    let [sender_info, recipient_info, mint_info, sender_token_account_info, recipient_token_account_info, token_program, associated_token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // validation
    sender_info.is_signer()?;
    mint_info.as_mint()?;
    sender_token_account_info
        .is_writable()?
        .as_associated_token_account(sender_info.key, mint_info.key)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;
    system_program.is_program(&system_program::ID)?;

    if recipient_token_account_info.lamports() == 0 {
        msg!("Creating associated token account for recipient...");
        create_associated_token_account(
            sender_info,
            recipient_info,
            recipient_token_account_info,
            mint_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    } else {
        msg!("Associated token account exists.");
    }
    msg!(
        "Recipient Associated Token Address: {}",
        recipient_token_account_info.key
    );

    msg!("Transferring {} tokens...", quantity);
    msg!("Mint: {}", mint_info.key);
    msg!("Owner Token Address: {}", sender_token_account_info.key);
    msg!(
        "Recipient Token Address: {}",
        recipient_token_account_info.key
    );

    transfer_spl_tokens(
        sender_info,
        sender_token_account_info,
        recipient_token_account_info,
        token_program,
        quantity,
    )?;

    msg!("Tokens transferred successfully.");

    Ok(())
}
