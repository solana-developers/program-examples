use pinocchio::{account_info::AccountInfo, pubkey::Pubkey, ProgramResult};
use pinocchio_log::logger::Logger;

use crate::{load, token2022_utils::has_immutable_owner_extension, BlockListError, WalletBlock};

///
/// SECURITY ASSUMPTIONS OVER TX-HOOK
/// 
/// 1- its called by the token-2022 program
/// 2- if some other program is calling it, we don't care as we don't write state here
/// 2- its inputs are already sanitized by the token-2022 program
/// 3- if some other program is calling it with invalid inputs, we don't care as we only read state and return ok/nok
/// 4- there may be 3 different extra metas setup
/// 4.1- no extra accounts
/// 4.2- only source wallet block
/// 4.3- both source and destination wallet blocks
/// 5- given all the above we can skip a lot of type and owner checks

pub struct TxHook<'a> {
    pub source: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub destination: &'a AccountInfo,
    pub authority: &'a AccountInfo,
    pub source_wallet_block: Option<&'a AccountInfo>,
    pub destination_wallet_block: Option<&'a AccountInfo>,
    //pub remaining_accounts: &'a [AccountInfo],
}

impl<'a> TxHook<'a> {
    pub const DISCRIMINATOR: u8 = 0x69;

    pub fn process(&self) -> ProgramResult {
        // check if there is a wallet block for the source account
        if let Some(source_wallet_block) = self.source_wallet_block {
            let source_data = unsafe {self.source.borrow_data_unchecked()};
            // without the immutable owner extension, TA owners could bypass wallet blocks
            // by changing the owner to a different wallet controlled by the same entity
            if !has_immutable_owner_extension(source_data) {
                let mut logger = Logger::<64>::default();
                logger.append("Transfer Blocked: Source TA - ImmutableOwnerExtensionMissing");
                logger.log();
                return Err(BlockListError::ImmutableOwnerExtensionMissing.into());
            }

            if !source_wallet_block.data_is_empty() {

                let _ = unsafe { load::<WalletBlock>(source_wallet_block.borrow_data_unchecked())? };

                // its a potential blocked wallet
                // lets check if authority is not the owner nor the delegate
                // this implies its the permanent delegate
                // alternatively we can decode the mint and get the permanent delegate

                let owner = unsafe { &*(source_data[32..64].as_ptr() as *const Pubkey) };
                let delegate = unsafe { &*(source_data[76..108].as_ptr() as *const Pubkey) };

                if owner.eq(self.authority.key()) || delegate.eq(self.authority.key()) {
                    let mut logger = Logger::<64>::default();
                    logger.append("Transfer Blocked:  Source TA - AccountBlocked");
                    logger.log();
                    return Err(BlockListError::AccountBlocked.into());
                }
                
            }

        }

        // check if there is a wallet block for the destination account
        if let Some(destination_wallet_block) = self.destination_wallet_block {

            if !has_immutable_owner_extension(unsafe {self.destination.borrow_data_unchecked()}) {
                let mut logger = Logger::<64>::default();
                logger.append("Transfer Blocked: Destination TA - ImmutableOwnerExtensionMissing");
                logger.log();
                return Err(BlockListError::ImmutableOwnerExtensionMissing.into());
            }

            if !destination_wallet_block.data_is_empty() {

                let _ = unsafe { load::<WalletBlock>(destination_wallet_block.borrow_data_unchecked())? };
                
                let mut logger = Logger::<64>::default();
                logger.append("Transfer Blocked:  Destination TA - AccountBlocked");
                logger.log();

                return Err(BlockListError::AccountBlocked.into());
            }

        }

        Ok(())
    }

}

impl<'a> TryFrom<&'a [AccountInfo]> for TxHook<'a> {
    type Error = BlockListError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {

        /*
        TX HOOK GETS CALLED WITH:
         1- source TA
         2- mint
         3- destination TA
         4- authority (either src owner or src delegate)
         5- extra account metas
         6- (optional) source wallet block 
         7- (optional) destination wallet block 
         */

        let [source, mint, destination, authority, remaining_accounts @ ..] = accounts else {
            return Err(BlockListError::NotEnoughAccounts);
        };

        let (source_wallet_block, destination_wallet_block) = if remaining_accounts.len() == 2 {
            (Some(&remaining_accounts[1]), None)
        } else if remaining_accounts.len() == 3 {
            (Some(&remaining_accounts[1]), Some(&remaining_accounts[2]))
        } else {
            (None, None)
        };



        Ok(Self {
            source,
            destination,
            mint,
            authority,
            source_wallet_block,
            destination_wallet_block,
        })
    }
}