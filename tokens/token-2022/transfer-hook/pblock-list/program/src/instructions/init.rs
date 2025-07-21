use pinocchio::{account_info::AccountInfo, instruction::Signer, pubkey::find_program_address, seeds, sysvars::{rent::Rent, Sysvar}, ProgramResult};

use crate::{load_mut_unchecked, BlockListError, Config, Discriminator, Transmutable};



pub struct Init<'a> {
    pub authority: &'a AccountInfo,
    pub config: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub config_bump: u8,
}

impl<'a> Discriminator for Init<'a> {
    const DISCRIMINATOR: u8 = 0xF1;
}

impl<'a> TryFrom<&'a [AccountInfo]> for Init<'a> {
    type Error = BlockListError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [authority, config, system_program] = accounts else {
            return Err(BlockListError::NotEnoughAccounts);
        };

        if !authority.is_signer() {
            return Err(BlockListError::InvalidAuthority);
        }

        /* do we really need to check this? its going to fail silently if not writable
        if !config.is_writable {
            return Err(BlockListError::InvalidInstruction);
        }*/


        // derive config account
        let (_, config_bump) = find_program_address(&[Config::SEED_PREFIX], &crate::ID);
        // no need to check if address is valid
        // cpi call with config as signer, runtime will check if the right account has been signer escalated

        //if config_account.ne(config.key()) {
        //    return Err(BlockListError::InvalidConfigAccount);
        //}

        // check if system program is valid
        if system_program.key().ne(&pinocchio_system::ID) {
            return Err(BlockListError::InvalidSystemProgram);
        }


        Ok(Self {
            authority,
            config,
            system_program,
            config_bump,
        })
    }
}

impl<'a> Init<'a> {
    pub fn process(&self) -> ProgramResult {
        let lamports = Rent::get()?.minimum_balance(Config::LEN);

        let bump_seed = [self.config_bump];
        let seeds = seeds!(Config::SEED_PREFIX, &bump_seed);
        let signer = Signer::from(&seeds);
            
        pinocchio_system::instructions::CreateAccount {
            from: self.authority,
            to: self.config,
            lamports,
            space: Config::LEN as u64,
            owner: &crate::ID,
        }.invoke_signed(&[signer])?;

        let mut data = self.config.try_borrow_mut_data()?;
        let config = unsafe { 
            load_mut_unchecked::<Config>(&mut data)? 
        };
        config.discriminator = Config::DISCRIMINATOR;
        config.authority = *self.authority.key();

        Ok(())
    }
}