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
    error::FundraiserError,
    instructions::SECONDS_TO_DAYS,
    state::{Contributor, Fundraiser},
};

/// Refunds a contributor after a failed fundraiser.
///
/// Once the campaign has ended without reaching its target, a contributor can
/// reclaim their tokens. Their contributed amount is returned from the vault and
/// the contributor account is closed (its rent returned to the contributor).
///
/// Accounts:
///   0. `[signer, writable]` contributor (receives the refund and reclaimed rent)
///   1. `[]`                 maker (part of the fundraiser PDA seeds)
///   2. `[]`                 mint to raise
///   3. `[writable]`         fundraiser account (PDA)
///   4. `[writable]`         contributor account (PDA, closed here)
///   5. `[writable]`         contributor's token account (receives the refund)
///   6. `[writable]`         vault (fundraiser's token account)
///   7. `[]`                 token program
///   8. `[]`                 system program
///
/// Instruction data: `[contributor_bump: u8]`
pub fn refund(program_id: &Address, accounts: &[AccountView], data: &[u8]) -> ProgramResult {
    let [contributor, maker, mint_to_raise, fundraiser, contributor_account, contributor_ata, vault, _token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !contributor.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let contributor_bump = *data.first().ok_or(ProgramError::InvalidInstructionData)?;

    // Load the campaign and confirm the maker, mint and PDA all match.
    let mut fundraiser_state = Fundraiser::deserialize(&fundraiser.try_borrow()?)?;
    if &fundraiser_state.maker != maker.address().as_array() {
        return Err(FundraiserError::InvalidSeeds.into());
    }
    if &fundraiser_state.mint_to_raise != mint_to_raise.address().as_array() {
        return Err(FundraiserError::InvalidMint.into());
    }
    let fundraiser_pda = derive_address(
        &[Fundraiser::SEED_PREFIX, maker.address().as_ref()],
        Some(fundraiser_state.bump),
        program_id.as_array(),
    );
    if fundraiser.address().as_array() != &fundraiser_pda {
        return Err(FundraiserError::InvalidSeeds.into());
    }

    // Confirm the contributor account is the canonical PDA for this contributor.
    let contributor_pda = derive_address(
        &[
            Contributor::SEED_PREFIX,
            fundraiser.address().as_ref(),
            contributor.address().as_ref(),
        ],
        Some(contributor_bump),
        program_id.as_array(),
    );
    if contributor_account.address().as_array() != &contributor_pda {
        return Err(FundraiserError::InvalidSeeds.into());
    }

    // Refunds are only allowed once the campaign has ended, i.e. after
    // `duration` days have elapsed since it started.
    let elapsed_days =
        ((Clock::get()?.unix_timestamp - fundraiser_state.time_started) / SECONDS_TO_DAYS) as u16;
    if elapsed_days < fundraiser_state.duration {
        return Err(FundraiserError::FundraiserNotEnded.into());
    }

    // The vault must be the fundraiser's token account for the raised mint.
    let vault_amount = {
        let vault_account = TokenAccount::from_account_view(vault)?;
        if vault_account.owner() != fundraiser.address()
            || vault_account.mint() != mint_to_raise.address()
        {
            return Err(FundraiserError::InvalidVault.into());
        }
        vault_account.amount()
    };

    // ...and only refund if the target was not met.
    if vault_amount >= fundraiser_state.amount_to_raise {
        return Err(FundraiserError::TargetMet.into());
    }

    let contributor_amount = Contributor::deserialize(&contributor_account.try_borrow()?)?.amount;

    // Return the contributor's tokens from the vault, signed by the fundraiser PDA.
    let bump_bytes = [fundraiser_state.bump];
    let seeds = [
        Seed::from(Fundraiser::SEED_PREFIX),
        Seed::from(maker.address().as_ref()),
        Seed::from(&bump_bytes),
    ];
    let signers = [Signer::from(&seeds)];

    log!("Refunding contribution from vault");
    Transfer {
        from: vault,
        to: contributor_ata,
        authority: fundraiser,
        amount: contributor_amount,
    }
    .invoke_signed(&signers)?;

    // Reduce the campaign's recorded total.
    fundraiser_state.current_amount -= contributor_amount;
    fundraiser_state.serialize(&mut fundraiser.try_borrow_mut()?)?;

    // Close the contributor account, returning its rent to the contributor.
    log!("Closing contributor account");
    let contributor_lamports = contributor_account.lamports();
    contributor_account.set_lamports(0);
    contributor.set_lamports(contributor.lamports() + contributor_lamports);
    contributor_account.resize(0)?;
    unsafe {
        contributor_account.assign(system_program.address());
    }

    log!("Refund completed successfully");
    Ok(())
}
