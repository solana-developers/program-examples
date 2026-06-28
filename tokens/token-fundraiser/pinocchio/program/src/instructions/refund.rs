use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    AccountView, Address, ProgramResult,
};
use pinocchio_log::log;
use pinocchio_pubkey::derive_address;
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::{
    constants::SECONDS_TO_DAYS,
    error::FundraiserError,
    state::{Contributor, Fundraiser},
};

/// Refunds a contributor after a fundraiser ends without meeting its target:
/// returns the contributor's deposit and closes the contributor account.
///
/// Accounts:
///   0. `[signer, writable]` contributor (receives the refund and reclaimed rent)
///   1. `[]`                 maker (part of the fundraiser PDA seeds)
///   2. `[]`                 mint to raise
///   3. `[writable]`         fundraiser account (PDA `[b"fundraiser", maker]`)
///   4. `[writable]`         contributor account (PDA `[b"contributor", fundraiser, contributor]`, closed here)
///   5. `[writable]`         contributor's token account (receives the refund)
///   6. `[writable]`         vault (fundraiser PDA's token account)
///   7. `[]`                 token program
///   8. `[]`                 system program
///
/// Instruction data: none.
pub fn refund(program_id: &Address, accounts: &[AccountView], _data: &[u8]) -> ProgramResult {
    let [contributor, maker, mint_to_raise, fundraiser, contributor_account, contributor_ata, vault, _token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !contributor.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Load the fundraiser and confirm it is the genuine PDA for this maker.
    let mut fundraiser_state = Fundraiser::deserialize(&fundraiser.try_borrow()?)?;
    let fundraiser_pda = derive_address(
        &[Fundraiser::SEED_PREFIX, maker.address().as_ref()],
        Some(fundraiser_state.bump),
        program_id.as_array(),
    );
    if fundraiser.address().as_array() != &fundraiser_pda {
        return Err(ProgramError::InvalidSeeds);
    }
    if &fundraiser_state.maker != maker.address().as_array()
        || &fundraiser_state.mint_to_raise != mint_to_raise.address().as_array()
    {
        return Err(ProgramError::InvalidAccountData);
    }

    // The fundraiser must have ended.
    let current_time = Clock::get()?.unix_timestamp;
    let elapsed_days = ((current_time - fundraiser_state.time_started) / SECONDS_TO_DAYS) as u16;
    if fundraiser_state.duration < elapsed_days {
        return Err(FundraiserError::FundraiserNotEnded.into());
    }

    // Refunds are only possible when the target was not met.
    let vault_amount = TokenAccount::from_account_view(vault)?.amount();
    if vault_amount >= fundraiser_state.amount_to_raise {
        return Err(FundraiserError::TargetMet.into());
    }

    // Load the contributor record and verify its PDA.
    let contributor_state = Contributor::deserialize(&contributor_account.try_borrow()?)?;
    let contributor_pda = derive_address(
        &[
            Contributor::SEED_PREFIX,
            fundraiser.address().as_ref(),
            contributor.address().as_ref(),
        ],
        Some(contributor_state.bump),
        program_id.as_array(),
    );
    if contributor_account.address().as_array() != &contributor_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    // Return the contributor's deposit, signed by the fundraiser PDA.
    let bump_bytes = [fundraiser_state.bump];
    let seeds = [
        Seed::from(Fundraiser::SEED_PREFIX),
        Seed::from(maker.address().as_ref()),
        Seed::from(&bump_bytes),
    ];
    let signers = [Signer::from(&seeds)];

    log!("Refunding contribution");
    Transfer {
        from: vault,
        to: contributor_ata,
        authority: fundraiser,
        amount: contributor_state.amount,
    }
    .invoke_signed(&signers)?;

    // Reduce the fundraiser's running total by the refunded amount.
    fundraiser_state.current_amount = fundraiser_state
        .current_amount
        .checked_sub(contributor_state.amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    fundraiser_state.serialize(&mut fundraiser.try_borrow_mut()?)?;

    // Close the contributor account, returning its rent to the contributor.
    log!("Closing contributor account");
    let contributor_account_lamports = contributor_account.lamports();
    contributor_account.set_lamports(0);
    contributor.set_lamports(contributor.lamports() + contributor_account_lamports);
    contributor_account.resize(0)?;
    unsafe {
        contributor_account.assign(system_program.address());
    }

    log!("Refund completed successfully");
    Ok(())
}
