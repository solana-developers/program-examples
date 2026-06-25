use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::{clock::Clock, rent::Rent, Sysvar},
    AccountView, Address, ProgramResult,
};
use pinocchio_log::log;
use pinocchio_pubkey::derive_address;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::{
    error::FundraiserError,
    instructions::{max_contribution, read_u64, SECONDS_TO_DAYS},
    state::{Contributor, Fundraiser},
};

/// Contributes tokens to a fundraiser.
///
/// Tokens are moved from the contributor's token account into the vault, and the
/// contributor's running total is recorded in a per-contributor PDA
/// (`[b"contributor", fundraiser, contributor]`), created on first contribution.
///
/// Accounts:
///   0. `[signer, writable]` contributor (funds the contributor account, sends tokens)
///   1. `[]`                 mint to raise
///   2. `[writable]`         fundraiser account (PDA)
///   3. `[writable]`         contributor account (PDA, created on first contribution)
///   4. `[writable]`         contributor's token account (source of the tokens)
///   5. `[writable]`         vault (fundraiser's token account)
///   6. `[]`                 token program
///   7. `[]`                 system program
///
/// Instruction data: `[amount: u64 (LE), contributor_bump: u8]`
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

    // Load the campaign and re-derive its PDA to confirm authenticity.
    let mut fundraiser_state = Fundraiser::deserialize(&fundraiser.try_borrow()?)?;
    if &fundraiser_state.mint_to_raise != mint_to_raise.address().as_array() {
        return Err(FundraiserError::InvalidMint.into());
    }
    let fundraiser_pda = derive_address(
        &[Fundraiser::SEED_PREFIX, fundraiser_state.maker.as_ref()],
        Some(fundraiser_state.bump),
        program_id.as_array(),
    );
    if fundraiser.address().as_array() != &fundraiser_pda {
        return Err(FundraiserError::InvalidSeeds.into());
    }

    // The vault must be a token account owned by the fundraiser for the raised
    // mint. Without this check a contributor could pass an account they control
    // as the vault, keep their tokens, and still inflate the recorded total.
    {
        let vault_account = TokenAccount::from_account_view(vault)?;
        if vault_account.owner() != fundraiser.address()
            || vault_account.mint() != mint_to_raise.address()
        {
            return Err(FundraiserError::InvalidVault.into());
        }
    }

    // A contribution must be non-zero (mirrors `1.pow(decimals) == 1`).
    if amount < 1 {
        return Err(FundraiserError::ContributionTooSmall.into());
    }

    // A single contribution cannot exceed the per-contributor maximum.
    let max = max_contribution(fundraiser_state.amount_to_raise);
    if amount > max {
        return Err(FundraiserError::ContributionTooBig.into());
    }

    // Contributions are only accepted while the campaign is still open, i.e.
    // before `duration` days have elapsed since it started.
    let elapsed_days =
        ((Clock::get()?.unix_timestamp - fundraiser_state.time_started) / SECONDS_TO_DAYS) as u16;
    if elapsed_days >= fundraiser_state.duration {
        return Err(FundraiserError::FundraiserEnded.into());
    }

    // Track this contributor's running total, creating their account if needed.
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

    // The contributor's total contribution cannot exceed the per-contributor max.
    if previous_amount > max || previous_amount + amount > max {
        return Err(FundraiserError::MaximumContributionsReached.into());
    }

    // Move the tokens into the vault.
    log!("Transferring contribution into vault");
    Transfer {
        from: contributor_ata,
        to: vault,
        authority: contributor,
        amount,
    }
    .invoke()?;

    // Update the campaign and contributor totals.
    fundraiser_state.current_amount += amount;
    fundraiser_state.serialize(&mut fundraiser.try_borrow_mut()?)?;

    let contributor_state = Contributor {
        amount: previous_amount + amount,
    };
    contributor_state.serialize(&mut contributor_account.try_borrow_mut()?)?;

    log!("Contribution recorded successfully");
    Ok(())
}
