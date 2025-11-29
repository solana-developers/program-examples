use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::{load, load_mut_unchecked, BlockListError, Config, Discriminator, WalletBlock};


pub struct UnblockWallet<'a> {
    pub authority: &'a AccountInfo,
    pub config: &'a AccountInfo,
    pub wallet_block: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
}

impl<'a> UnblockWallet<'a> {
    pub fn process(&self) -> ProgramResult {
        
        let destination_lamports = self.authority.lamports();

        unsafe {
            *self.authority.borrow_mut_lamports_unchecked() = destination_lamports
                .checked_add(self.wallet_block.lamports())
                .ok_or(ProgramError::ArithmeticOverflow)?;
            self.wallet_block.close_unchecked();
        }
        
        let config = unsafe { load_mut_unchecked::<Config>(self.config.borrow_mut_data_unchecked())? };
        config.blocked_wallets_count = config.blocked_wallets_count.checked_sub(1).ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }
}

impl<'a> Discriminator for UnblockWallet<'a> {
    const DISCRIMINATOR: u8 = 0xF3;
}

impl<'a> TryFrom<&'a [AccountInfo]> for UnblockWallet<'a> {
    type Error = BlockListError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [authority, config, wallet_block, system_program] = accounts else {
            return Err(BlockListError::NotEnoughAccounts);
        };

        let cfg = unsafe { load::<Config>(config.borrow_data_unchecked())? };
        
        if !config.is_owned_by(&crate::ID) {
            return Err(BlockListError::InvalidConfigAccount);
        }

        if !authority.is_signer() || cfg.authority.ne(authority.key()) {
            return Err(BlockListError::InvalidAuthority);
        }
        
        if !config.is_writable() && !wallet_block.is_writable() {
            return Err(BlockListError::AccountNotWritable);
        }

        if unsafe { load::<WalletBlock>(wallet_block.borrow_data_unchecked()).is_err() }{
            return Err(BlockListError::InvalidAccountData);
        }

        Ok(Self {
            authority,
            config,
            wallet_block,
            system_program,
        })
    }
}