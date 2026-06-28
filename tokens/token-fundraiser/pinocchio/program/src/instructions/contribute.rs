use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::{clock::Clock, rent::Rent, Sysvar},
    AccountView, Address, ProgramResult,
};
use pinocchio_log::log;
use pinocchio_pubkey::derive_address;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::instructions::Transfer;

use crate::{
    constants::{MAX_CONTRIBUTION_PERCENTAGE, PERCENTAGE_SCALER, SECONDS_TO_DAYS},
    error::FundraiserError,
    instructions::read_u64,
    state::{Contributor, Fundraiser},
};

/// Contributes `amount` of the target token into the fundraiser vault, creating
/// the per-contributor record on first use.
///
/// Accounts:
///   0. `[signer, writable]` contributor (pays for the contributor account; source authority)
///   1. `[]`                 mint to raise
///   2. `[writable]`         fundraiser account (PDA `[b"fundraiser", maker]`)
///   3. `[writable]`         contributor account (PDA `[b"contributor", fundraiser, contributor]`, created if needed)
///   4. `[writable]`         contributor's token account (source of the deposit)
///   5. `[writable]`         vault (fundraiser PDA's token account)
///   6. `[]`                 token program
///   7. `[]`                 system program
///
/// Instruction data: `[amount: u64 (LE), bump: u8]` where `bump` is the
/// contributor PDA bump.
pub fn contribute(program_id: &Address, accounts: &[AccountView], data: &[u8]) -> ProgramResult {
    let [contributor, mint_to_raise, fundraiser, contributor_account, contributor_ata, vault, _token_program, _system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !contributor.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let amount = read_u64(data, 0)?;
    let contributor_bump = *data.get(8).ok_or(ProgramError::InvalidInstructionData)?;

    // Load the fundraiser and confirm it is the genuine PDA for the recorded maker.
    let mut fundraiser_state = Fundraiser::deserialize(&fundraiser.try_borrow()?)?;
    let fundraiser_pda = derive_address(
        &[Fundraiser::SEED_PREFIX, fundraiser_state.maker.as_ref()],
        Some(fundraiser_state.bump),
        program_id.as_array(),
    );
    if fundraiser.address().as_array() != &fundraiser_pda {
        return Err(ProgramError::InvalidSeeds);
    }
    if &fundraiser_state.mint_to_raise != mint_to_raise.address().as_array() {
        return Err(ProgramError::InvalidAccountData);
    }
    // The vault must be the fundraiser's recorded vault, otherwise a caller
    // could record a contribution against an account they control.
    if &fundraiser_state.vault != vault.address().as_array() {
        return Err(ProgramError::InvalidAccountData);
    }

    // A contribution must be at least one base unit.
    if amount < 1 {
        return Err(FundraiserError::ContributionTooSmall.into());
    }

    // No single contribution may exceed the per-contributor cap.
    let max_contribution = fundraiser_state
        .amount_to_raise
        .checked_mul(MAX_CONTRIBUTION_PERCENTAGE)
        .ok_or(ProgramError::ArithmeticOverflow)?
        / PERCENTAGE_SCALER;
    if amount > max_contribution {
        return Err(FundraiserError::ContributionTooBig.into());
    }

    // The fundraiser must still be within its active window.
    let current_time = Clock::get()?.unix_timestamp;
    let elapsed_days = ((current_time - fundraiser_state.time_started) / SECONDS_TO_DAYS) as u16;
    if elapsed_days > fundraiser_state.duration {
        return Err(FundraiserError::FundraiserEnded.into());
    }

    // Verify the contributor PDA and load (or initialize) the running total.
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
        return Err(ProgramError::InvalidSeeds);
    }

    let previous_amount = if contributor_account.data_len() == 0 {
        // First contribution: create the contributor account, signed by its PDA.
        let lamports = Rent::get()?.try_minimum_balance(Contributor::LEN)?;
        let bump_bytes = [contributor_bump];
        let seeds = [
            Seed::from(Contributor::SEED_PREFIX),
            Seed::from(fundraiser.address().as_ref()),
            Seed::from(contributor.address().as_ref()),
            Seed::from(&bump_bytes),
        ];
        let signers = [Signer::from(&seeds)];

        log!("Creating contributor account");
        CreateAccount {
            from: contributor,
            to: contributor_account,
            lamports,
            space: Contributor::LEN as u64,
            owner: program_id,
        }
        .invoke_signed(&signers)?;
        0
    } else {
        Contributor::deserialize(&contributor_account.try_borrow()?)?.amount
    };

    // The contributor's running total must not exceed the cap.
    let new_amount = previous_amount
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    if previous_amount > max_contribution || new_amount > max_contribution {
        return Err(FundraiserError::MaximumContributionsReached.into());
    }

    // Move the contributor's tokens into the vault.
    log!("Transferring contribution into vault");
    Transfer {
        from: contributor_ata,
        to: vault,
        authority: contributor,
        amount,
    }
    .invoke()?;

    // Update the running totals.
    fundraiser_state.current_amount = fundraiser_state
        .current_amount
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    fundraiser_state.serialize(&mut fundraiser.try_borrow_mut()?)?;

    let contributor_state = Contributor {
        amount: new_amount,
        bump: contributor_bump,
    };
    contributor_state.serialize(&mut contributor_account.try_borrow_mut()?)?;

    log!("Contribution recorded successfully");
    Ok(())
}
