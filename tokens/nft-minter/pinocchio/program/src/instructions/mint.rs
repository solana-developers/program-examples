use alloc::vec::Vec;

use pinocchio::{
    cpi::invoke,
    error::ProgramError,
    instruction::{InstructionAccount, InstructionView},
    AccountView, ProgramResult,
};
use pinocchio_associated_token_account::instructions::CreateIdempotent;
use pinocchio_log::log;
use pinocchio_token::instructions::MintTo;

use crate::instructions::TOKEN_METADATA_PROGRAM_ID;

/// Discriminator of the Metaplex `CreateMasterEditionV3` instruction (variant 17
/// of the Token Metadata program's instruction enum).
const CREATE_MASTER_EDITION_V3: u8 = 17;

/// Mints the single NFT token to the payer's associated token account and then
/// creates its master edition account. Creating the master edition transfers the
/// mint and freeze authorities to the edition PDA, capping the supply at one and
/// making this a true non-fungible token.
///
/// Accounts:
///   0. `[writable]`         mint account
///   1. `[writable]`         metadata account
///   2. `[writable]`         master edition account (the edition PDA)
///   3. `[]`                 mint authority (also the metadata update authority)
///   4. `[writable]`         payer's associated token account (the destination)
///   5. `[signer, writable]` payer (funds the accounts and owns the NFT)
///   6. `[]`                 system program
///   7. `[]`                 token program
///   8. `[]`                 associated token program
///   9. `[]`                 token metadata program
///
/// Instruction data: none.
///
/// The mint authority is passed as a non-signer; the `MintTo` and master edition
/// CPIs require it to sign, which is satisfied by passing the payer's address for
/// it (the payer signs the transaction). This mirrors the `native` example.
pub fn mint_to(accounts: &[AccountView]) -> ProgramResult {
    // `associated_token_program` and `token_metadata_program` are unused
    // directly, but must be supplied so they are present for the CPIs below.
    let [mint_account, metadata_account, edition_account, mint_authority, associated_token_account, payer, system_program, token_program, _associated_token_program, _token_metadata_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

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

    log!("Minting NFT to associated token account");
    MintTo {
        mint: mint_account,
        account: associated_token_account,
        mint_authority,
        amount: 1,
    }
    .invoke()?;

    // Create the master edition. This is what makes the token a real NFT: the
    // Token Metadata program takes over the mint and freeze authorities, so no
    // further tokens can ever be minted from this mint.
    log!("Creating master edition account");
    let edition_data = build_master_edition_data();
    let edition_accounts = [
        InstructionAccount::writable(edition_account.address()),
        InstructionAccount::writable(mint_account.address()),
        // Update authority and mint authority are the same key here.
        InstructionAccount::readonly_signer(mint_authority.address()),
        InstructionAccount::readonly_signer(mint_authority.address()),
        InstructionAccount::writable_signer(payer.address()),
        InstructionAccount::writable(metadata_account.address()),
        InstructionAccount::readonly(token_program.address()),
        InstructionAccount::readonly(system_program.address()),
    ];
    let instruction = InstructionView {
        program_id: &TOKEN_METADATA_PROGRAM_ID,
        accounts: &edition_accounts,
        data: &edition_data,
    };
    invoke(
        &instruction,
        &[
            edition_account,
            mint_account,
            mint_authority,
            mint_authority,
            payer,
            metadata_account,
            token_program,
            system_program,
        ],
    )?;

    log!("NFT minted successfully");
    Ok(())
}

/// Serializes the data for a Metaplex `CreateMasterEditionV3` instruction.
///
/// Layout: `[17] max_supply:Option<u64>`. A `max_supply` of `Some(1)` caps the
/// number of printable editions, mirroring the `native` example.
fn build_master_edition_data() -> Vec<u8> {
    let mut data = Vec::new();
    data.push(CREATE_MASTER_EDITION_V3);
    data.push(1); // max_supply: Some
    data.extend_from_slice(&1u64.to_le_bytes()); // max_supply value
    data
}
