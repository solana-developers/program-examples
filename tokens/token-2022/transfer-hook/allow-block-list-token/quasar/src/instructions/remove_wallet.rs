use quasar_lang::prelude::*;

use crate::constants::CONFIG_SEED;
use crate::errors;
use crate::state::{read_config_authority, CONFIG_SIZE};

#[derive(Accounts)]
pub struct RemoveWallet<'info> {
    #[account(mut)]
    pub authority: &'info Signer,
    pub config: &'info UncheckedAccount,
    #[account(mut)]
    pub ab_wallet: &'info mut UncheckedAccount,
}

#[inline(always)]
pub fn handle_remove_wallet(accounts: &RemoveWallet) -> Result<(), ProgramError> {
    // Verify config PDA
    let (config_pda, _) = Address::find_program_address(&[CONFIG_SEED], &crate::ID);
    if accounts.config.to_account_view().address() != &config_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    // Verify authority
    let config_view = accounts.config.to_account_view();
    let config_data = config_view.try_borrow()?;
    if config_data.len() < CONFIG_SIZE as usize {
        return Err(ProgramError::UninitializedAccount);
    }
    let stored_authority = read_config_authority(&config_data);
    if stored_authority != accounts.authority.to_account_view().address().as_ref() {
        return Err(errors::unauthorized());
    }
    drop(config_data);

    // Close the ABWallet account: transfer all lamports to authority, zero data.
    let wallet_view = accounts.ab_wallet.to_account_view();
    let wallet_lamports = wallet_view.lamports();
    let authority_view = accounts.authority.to_account_view();

    // Move lamports: drain wallet, credit authority
    set_lamports(wallet_view, 0);
    set_lamports(authority_view, authority_view.lamports() + wallet_lamports);

    // Zero the account data
    let mview = unsafe {
        &mut *(accounts.ab_wallet as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = mview.try_borrow_mut()?;
    for byte in data.iter_mut() {
        *byte = 0;
    }

    log("Wallet entry removed");
    Ok(())
}
