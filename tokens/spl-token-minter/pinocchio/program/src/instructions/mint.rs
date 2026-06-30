use pinocchio::{error::ProgramError, AccountView, ProgramResult};
use pinocchio_associated_token_account::instructions::CreateIdempotent;
use pinocchio_log::log;
use pinocchio_token::instructions::MintTo;

use crate::instructions::parse_u64;

/// Mints tokens into the payer's associated token account, creating that account
/// first if it does not already exist.
///
/// Accounts:
///   0. `[writable]`         mint account
///   1. `[]`                 mint authority
///   2. `[writable]`         payer's associated token account (the destination)
///   3. `[signer, writable]` payer (funds the associated token account and owns it)
///   4. `[]`                 system program
///   5. `[]`                 token program
///   6. `[]`                 associated token program
///
/// Instruction data: `[quantity: u64 (LE)]`.
///
/// The mint authority is passed as a non-signer; `MintTo` requires it to sign,
/// which is satisfied by passing the payer's address for it (the payer signs the
/// transaction). This mirrors the `native` example.
pub fn mint_tokens(accounts: &[AccountView], data: &[u8]) -> ProgramResult {
    let [mint_account, mint_authority, associated_token_account, payer, system_program, token_program, _associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let quantity = parse_u64(data)?;

    log!("Creating associated token account if needed");
    CreateIdempotent {
        funding_account: payer,
        account: associated_token_account,
        wallet: payer,
        mint: mint_account,
        system_program,
        token_program,
    }
    .invoke()?;

    log!("Minting tokens to associated token account");
    MintTo {
        mint: mint_account,
        account: associated_token_account,
        mint_authority,
        amount: quantity,
    }
    .invoke()?;

    log!("Tokens minted successfully");
    Ok(())
}
