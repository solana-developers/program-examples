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
    error::FundraiserError,
    instructions::{read_u64, MIN_AMOUNT_TO_RAISE},
    state::Fundraiser,
};

/// Creates a fundraiser and its vault.
///
/// The fundraiser account is a PDA (`[b"fundraiser", maker]`) that records the
/// campaign terms and owns the vault, an associated token account for the
/// raised mint.
///
/// Accounts:
///   0. `[signer, writable]` maker (creates and funds the fundraiser + vault)
///   1. `[]`                 mint to raise
///   2. `[writable]`         fundraiser account (PDA, created here)
///   3. `[writable]`         vault (fundraiser's associated token account, created here)
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
    let duration = u16::from_le_bytes(
        data.get(8..10)
            .ok_or(ProgramError::InvalidInstructionData)?
            .try_into()
            .unwrap(),
    );
    let bump = *data.get(10).ok_or(ProgramError::InvalidInstructionData)?;

    // The target must meet the minimum, scaled by the mint's decimals. This
    // mirrors the Anchor/native versions: `MIN_AMOUNT_TO_RAISE.pow(decimals)`.
    let decimals = Mint::from_account_view(mint_to_raise)?.decimals();
    if amount < MIN_AMOUNT_TO_RAISE.pow(decimals as u32) {
        return Err(FundraiserError::InvalidAmount.into());
    }

    // Confirm the supplied fundraiser account is the canonical PDA for the maker.
    let fundraiser_pda = derive_address(
        &[Fundraiser::SEED_PREFIX, maker.address().as_ref()],
        Some(bump),
        program_id.as_array(),
    );
    if fundraiser.address().as_array() != &fundraiser_pda {
        return Err(FundraiserError::InvalidSeeds.into());
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

    // Create the vault: an associated token account for the raised mint, owned
    // by the fundraiser PDA.
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

    // Persist the campaign terms.
    let fundraiser_state = Fundraiser {
        maker: *maker.address().as_array(),
        mint_to_raise: *mint_to_raise.address().as_array(),
        // The associated token program guaranteed this is the canonical ATA when
        // it was created above, so recording it lets later instructions reject
        // any substitute vault.
        vault: *vault.address().as_array(),
        amount_to_raise: amount,
        current_amount: 0,
        time_started: Clock::get()?.unix_timestamp,
        duration,
        bump,
    };
    fundraiser_state.serialize(&mut fundraiser.try_borrow_mut()?)?;

    log!("Fundraiser initialized successfully");
    Ok(())
}
