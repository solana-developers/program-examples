use crate::state::*;
use steel::*;
use sysvar::rent::Rent;

use super::SteelInstruction;

instruction!(SteelInstruction, ExtendAddressInfo);
/// extend address info
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C, packed)]
pub struct ExtendAddressInfo {
    pub address_info: EnhancedAddressInfoExtender,
}

impl ExtendAddressInfo {
    pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        // Steel uses zero_copy types, so the sizes are fixed, however we can move from one account
        // type to another, e.g AddressInfo -> ExtendedAddressInfo

        let [payer, address_info_account, _system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // we collect the extended data
        //
        let address_info_extended_data = Self::try_from_bytes(data)?.address_info;

        // We get pay for the extra space we need before allocating the space
        //
        let account_span = 8 + std::mem::size_of::<ExtendedAddressInfo>(); // 8 byte Account Discriminator + sizeof ExtendedAddressInfo
        let lamports_required = (Rent::get()?).minimum_balance(account_span);
        let diff = lamports_required.saturating_sub(address_info_account.lamports());
        address_info_account.collect(diff, payer)?;

        // we reallocate new space to accomodate the new data
        //
        address_info_account.realloc(account_span, false)?; // no zero init

        // we set the discriminator to the `ExtendedAccountInfo`, so Steel can deserialize the account as such.
        //
        {
            let mut data = address_info_account.data.borrow_mut();
            data[0] = ExtendedAddressInfo::discriminator();
        }

        // now we reset the account discriminator, we can deserialise as `ExtendedAddressInfo`
        //
        let extended_address_info =
            address_info_account.as_account_mut::<ExtendedAddressInfo>(&crate::ID)?;

        // set the extended address info
        //
        extended_address_info.state = address_info_extended_data.state;
        extended_address_info.zip = address_info_extended_data.zip;

        Ok(())
    }
}
