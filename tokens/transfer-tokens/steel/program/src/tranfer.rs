use solana_program::msg;
use steel::{transfer as transfer_spl_tokens, *};
use steel_api::prelude::*;

pub fn process_transfer(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // parse args.
    let args = Mint::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, mint_info, sender_info, recipient_info, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // validation
    signer_info.is_signer()?;
    mint_info.to_mint()?;
    token_program.is_program(&spl_token::ID)?;

    sender_info
        .is_writable()?
        .to_token_account()?
        .check(|t| t.owner == *signer_info.key)?;

    msg!("Transferring tokens...");
    msg!("Mint: {}", mint_info.key);
    msg!("From Token Address: {}", sender_info.key);
    msg!("To Token Address: {}", recipient_info.key);

    transfer_spl_tokens(
        signer_info,
        sender_info,
        recipient_info,
        token_program,
        amount,
    )?;

    msg!("Tokens transferred successfully.");

    Ok(())
}
