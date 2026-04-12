use quasar_lang::cpi::Seed;
use quasar_lang::prelude::*;
use quasar_lang::sysvars::Sysvar;

use crate::constants::CONFIG_SEED;
use crate::state::{write_config, CONFIG_SIZE};

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    #[account(mut)]
    pub config: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

impl InitConfig<'_> {
    #[inline(always)]
    pub fn init_config(&self) -> Result<(), ProgramError> {
        let (config_pda, bump) = Address::find_program_address(&[CONFIG_SEED], &crate::ID);

        if self.config.to_account_view().address() != &config_pda {
            return Err(ProgramError::InvalidSeeds);
        }

        let lamports = Rent::get()?.try_minimum_balance(CONFIG_SIZE as usize)?;
        let bump_bytes = [bump];
        let seeds = [
            Seed::from(CONFIG_SEED),
            Seed::from(&bump_bytes as &[u8]),
        ];

        self.system_program
            .create_account(
                self.payer,
                &*self.config,
                lamports,
                CONFIG_SIZE,
                &crate::ID,
            )
            .invoke_signed(&seeds)?;

        let view = unsafe {
            &mut *(self.config as *const UncheckedAccount as *mut UncheckedAccount
                as *mut AccountView)
        };
        let mut data = view.try_borrow_mut()?;

        write_config(&mut data, self.payer.to_account_view().address(), bump);

        log("Config initialized");
        Ok(())
    }
}
