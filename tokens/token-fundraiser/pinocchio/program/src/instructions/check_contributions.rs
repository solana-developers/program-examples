use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    AccountView, Address, ProgramResult,
};
use pinocchio_associated_token_account::instructions::CreateIdempotent;
use pinocchio_log::log;
use pinocchio_pubkey::derive_address;
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::{error::FundraiserError, state::Fundraiser};

/// Settles a successful fundraiser.
///
/// If the vault holds at least the target amount, all funds are transferred to
/// the maker and the fundraiser account is closed (its rent returned to the
/// maker).
///
/// Accounts:
///   0. `[signer, writable]` maker (receives the funds and the reclaimed rent)
///   1. `[]`                 mint to raise
///   2. `[writable]`         fundraiser account (PDA, closed here)
///   3. `[writable]`         vault (fundraiser's token account, drained here)
///   4. `[writable]`         maker's token account (created if needed)
///   5. `[]`                 token program
///   6. `[]`                 associated token program
///   7. `[]`                 system program
///
/// Instruction data: none.
pub fn check_contributions(
    program_id: &Address,
    accounts: &[AccountView],
    _data: &[u8],
) -> ProgramResult {
    let [maker, mint_to_raise, fundraiser, vault, maker_ata, token_program, _associated_token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Load the campaign and confirm the maker, mint and PDA all match.
    let fundraiser_state = Fundraiser::deserialize(&fundraiser.try_borrow()?)?;
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

    // The campaign must have reached its target.
    if vault_amount < fundraiser_state.amount_to_raise {
        return Err(FundraiserError::TargetNotMet.into());
    }

    // Make sure the maker has a token account to receive into.
    log!("Ensuring maker token account exists");
    CreateIdempotent {
        funding_account: maker,
        account: maker_ata,
        wallet: maker,
        mint: mint_to_raise,
        system_program,
        token_program,
    }
    .invoke()?;

    // Release the raised funds to the maker, signed by the fundraiser PDA.
    let bump_bytes = [fundraiser_state.bump];
    let seeds = [
        Seed::from(Fundraiser::SEED_PREFIX),
        Seed::from(maker.address().as_ref()),
        Seed::from(&bump_bytes),
    ];
    let signers = [Signer::from(&seeds)];

    log!("Transferring raised funds to maker");
    Transfer {
        from: vault,
        to: maker_ata,
        authority: fundraiser,
        amount: vault_amount,
    }
    .invoke_signed(&signers)?;

    // Close the fundraiser account, returning its rent to the maker.
    log!("Closing fundraiser account");
    let fundraiser_lamports = fundraiser.lamports();
    fundraiser.set_lamports(0);
    maker.set_lamports(maker.lamports() + fundraiser_lamports);
    fundraiser.resize(0)?;
    unsafe {
        fundraiser.assign(system_program.address());
    }

    log!("Fundraiser settled successfully");
    Ok(())
}
