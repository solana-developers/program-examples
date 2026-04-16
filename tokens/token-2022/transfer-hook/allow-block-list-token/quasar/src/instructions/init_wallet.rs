use quasar_lang::cpi::Seed;
use quasar_lang::prelude::*;
use quasar_lang::sysvars::Sysvar;

use crate::constants::{AB_WALLET_SEED, CONFIG_SEED};
use crate::errors;
use crate::state::{read_config_authority, write_ab_wallet, AB_WALLET_SIZE, CONFIG_SIZE};

#[derive(Accounts)]
pub struct InitWallet<'info> {
    #[account(mut)]
    pub authority: &'info Signer,
    pub config: &'info UncheckedAccount,
    pub wallet: &'info UncheckedAccount,
    #[account(mut)]
    pub ab_wallet: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_init_wallet(accounts: &InitWallet, allowed: bool) -> Result<(), ProgramError> {
    // Verify config PDA
    let (config_pda, _) = Address::find_program_address(&[CONFIG_SEED], &crate::ID);
    if accounts.config.to_account_view().address() != &config_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    // Verify authority matches config
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

    // Create ABWallet PDA
    let wallet_key = accounts.wallet.to_account_view().address();
    let (ab_wallet_pda, bump) = Address::find_program_address(
        &[AB_WALLET_SEED, wallet_key.as_ref()],
        &crate::ID,
    );
    if accounts.ab_wallet.to_account_view().address() != &ab_wallet_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    let lamports = Rent::get()?.try_minimum_balance(AB_WALLET_SIZE as usize)?;
    let bump_bytes = [bump];
    let seeds = [
        Seed::from(AB_WALLET_SEED),
        Seed::from(wallet_key.as_ref()),
        Seed::from(&bump_bytes as &[u8]),
    ];

    accounts.system_program
        .create_account(
            accounts.authority,
            &*accounts.ab_wallet,
            lamports,
            AB_WALLET_SIZE,
            &crate::ID,
        )
        .invoke_signed(&seeds)?;

    // Write wallet data
    let view = unsafe {
        &mut *(accounts.ab_wallet as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = view.try_borrow_mut()?;
    write_ab_wallet(&mut data, wallet_key, allowed);

    log("Wallet entry created");
    Ok(())
}
