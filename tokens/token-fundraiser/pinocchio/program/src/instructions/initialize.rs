use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::{clock::Clock, rent::Rent, Sysvar},
    AccountView, Address, ProgramResult,
};
use pinocchio_associated_token_account::instructions::CreateIdempotent;
use pinocchio_log::log;
use pinocchio_pubkey::derive_address;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::state::Mint;

use crate::{
    constants::MIN_AMOUNT_TO_RAISE,
    error::FundraiserError,
    instructions::{read_u16, read_u64},
    state::Fundraiser,
};

/// Creates a fundraiser: records the target mint and amount and opens a vault
/// (the fundraiser PDA's associated token account) to collect contributions.
///
/// Accounts:
///   0. `[signer, writable]` maker (pays for the new accounts; the fundraiser authority)
///   1. `[]`                 mint to raise
///   2. `[writable]`         fundraiser account (PDA `[b"fundraiser", maker]`, created here)
///   3. `[writable]`         vault (fundraiser PDA's associated token account, created here)
///   4. `[]`                 token program
///   5. `[]`                 associated token program
///   6. `[]`                 system program
///
/// Instruction data: `[amount: u64 (LE), duration: u16 (LE), bump: u8]`
pub fn initialize(program_id: &Address, accounts: &[AccountView], data: &[u8]) -> ProgramResult {
    let [maker, mint_to_raise, fundraiser, vault, token_program, _associated_token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let amount = read_u64(data, 0)?;
    let duration = read_u16(data, 8)?;
    let bump = *data.get(10).ok_or(ProgramError::InvalidInstructionData)?;

    // The target must clear the decimals-scaled minimum (`3^decimals`).
    let decimals = Mint::from_account_view(mint_to_raise)?.decimals();
    let min_amount = MIN_AMOUNT_TO_RAISE
        .checked_pow(decimals as u32)
        .ok_or(FundraiserError::InvalidAmount)?;
    if amount < min_amount {
        return Err(FundraiserError::InvalidAmount.into());
    }

    // Verify the supplied fundraiser account is the canonical PDA.
    let fundraiser_pda = derive_address(
        &[Fundraiser::SEED_PREFIX, maker.address().as_ref()],
        Some(bump),
        program_id.as_array(),
    );
    if fundraiser.address().as_array() != &fundraiser_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    // Create the fundraiser account, signed by the fundraiser PDA itself.
    let lamports = Rent::get()?.try_minimum_balance(Fundraiser::LEN)?;
    let bump_bytes = [bump];
    let seeds = [
        Seed::from(Fundraiser::SEED_PREFIX),
        Seed::from(maker.address().as_ref()),
        Seed::from(&bump_bytes),
    ];
    let signers = [Signer::from(&seeds)];

    log!("Creating fundraiser account");
    CreateAccount {
        from: maker,
        to: fundraiser,
        lamports,
        space: Fundraiser::LEN as u64,
        owner: program_id,
    }
    .invoke_signed(&signers)?;

    // Create the vault: an associated token account for the mint, owned by the
    // fundraiser PDA.
    log!("Creating vault");
    CreateIdempotent {
        funding_account: maker,
        account: vault,
        wallet: fundraiser,
        mint: mint_to_raise,
        system_program,
        token_program,
    }
    .invoke()?;

    // Persist the fundraiser terms.
    let fundraiser_state = Fundraiser {
        maker: *maker.address().as_array(),
        mint_to_raise: *mint_to_raise.address().as_array(),
        amount_to_raise: amount,
        current_amount: 0,
        time_started: Clock::get()?.unix_timestamp,
        duration,
        bump,
        vault: *vault.address().as_array(),
    };
    fundraiser_state.serialize(&mut fundraiser.try_borrow_mut()?)?;

    log!("Fundraiser created successfully");
    Ok(())
}
