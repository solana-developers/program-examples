use crate::state::*;
use steel::*;
use sysvar::rent::Rent;

use super::SteelInstruction;

instruction!(SteelInstruction, ExtendAddressInfo);
/// create address info
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

        let address_info = address_info_account.as_account::<AddressInfo>(&crate::ID)?;

        // We collect our current address data and combime with the
        //  data extension
        //
        let extended_address_info_data = ExtendedAddressInfo {
            name: address_info.name,
            house_number: address_info.house_number,
            street: address_info.street,
            city: address_info.city,
            zip: address_info_extended_data.zip,
            state: address_info_extended_data.state,
        };

        // We get pay for the extra space we need before allocating the space
        //
        let account_span = 8 + (extended_address_info_data.to_bytes()).len();
        let lamports_required = (Rent::get()?).minimum_balance(account_span);
        let diff = lamports_required - address_info_account.lamports();

        // transfer the difference
        address_info_account.collect(diff, payer)?;

        // we reallocate new space to accomodate the new data
        //
        address_info_account.realloc(account_span, false)?;

        // we set the discriminator to the `ExtendedAccountInfo`, so Steel can deserialize.
        // the account as such.
        //
        {
            let mut data = address_info_account.data.borrow_mut();
            // reset the account discriminator
            data[0] = ExtendAddressInfo::discriminator();
        }

        // now we reset the account discriminator, we can deserialise as `ExtendedAddressInfo
        //
        let extended_address_info =
            address_info_account.as_account_mut::<ExtendedAddressInfo>(&crate::ID)?;

        // set the extended address info
        //
        *extended_address_info = extended_address_info_data;

        Ok(())
    }
}
