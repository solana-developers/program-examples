use pinocchio::{account_info::AccountInfo, instruction::Signer, program_error::ProgramError, pubkey::find_program_address, seeds, sysvars::{rent::Rent, Sysvar}, ProgramResult};

use crate::{load, load_mut_unchecked, BlockListError, Config, Discriminator, Transmutable, WalletBlock};


pub struct BlockWallet<'a> {
    pub authority: &'a AccountInfo,
    pub config: &'a AccountInfo,
    pub wallet: &'a AccountInfo,
    pub wallet_block: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub wallet_block_bump: u8,
}

impl<'a> BlockWallet<'a> {
    pub fn process(&self) -> ProgramResult {
        let lamports = Rent::get()?.minimum_balance(WalletBlock::LEN);

        let bump_seed = [self.wallet_block_bump];
        let seeds = seeds!(WalletBlock::SEED_PREFIX, self.wallet.key(), &bump_seed);
        let signer = Signer::from(&seeds);
            
        pinocchio_system::instructions::CreateAccount {
            from: self.authority,
            to: self.wallet_block,
            lamports,
            space: WalletBlock::LEN as u64,
            owner: &crate::ID,
        }.invoke_signed(&[signer])?;

        let mut data = self.wallet_block.try_borrow_mut_data()?;
        let wallet_block = unsafe { 
            load_mut_unchecked::<WalletBlock>(&mut data)? 
        };
        wallet_block.discriminator = WalletBlock::DISCRIMINATOR;
        wallet_block.address = *self.wallet.key();

        let config = unsafe { load_mut_unchecked::<Config>(self.config.borrow_mut_data_unchecked())? };
        config.blocked_wallets_count = config.blocked_wallets_count.checked_add(1).ok_or(ProgramError::ArithmeticOverflow)?;
        
        Ok(())
    }
}

impl<'a> Discriminator for BlockWallet<'a> {
    const DISCRIMINATOR: u8 = 0xF2;
}

impl<'a> TryFrom<&'a [AccountInfo]> for BlockWallet<'a> {
    type Error = BlockListError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [authority, config, wallet, wallet_block, system_program] = accounts else {
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

        let (_, wallet_block_bump) = find_program_address(&[WalletBlock::SEED_PREFIX, wallet.key()], &crate::ID);

        // check if system program is valid
        if system_program.key().ne(&pinocchio_system::ID) {
            return Err(BlockListError::InvalidSystemProgram);
        }

        Ok(Self {
            authority,
            config,
            wallet,
            wallet_block,
            system_program,
            wallet_block_bump,
        })
    }
}