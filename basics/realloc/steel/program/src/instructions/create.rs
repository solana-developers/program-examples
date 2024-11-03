use crate::state::AddressInfo;
use steel::*;

use super::SteelInstruction;

instruction!(SteelInstruction, CreateAddressInfo);
/// create address info
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C, packed)]
pub struct CreateAddressInfo {
    pub address_info: AddressInfo,
}

impl CreateAddressInfo {
    pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        let address_info_data = Self::try_from_bytes(data)?.address_info;

        let [payer, address_info_account, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        payer.is_signer()?;
        address_info_account.is_signer()?;
        address_info_account.is_empty()?;
        system_program.is_program(&system_program::ID)?;

        create_account::<AddressInfo>(
            address_info_account,
            system_program,
            payer,
            &crate::ID,
            &[],
        )?;

        let address_info = address_info_account.as_account_mut::<AddressInfo>(&crate::ID)?;

        *address_info = address_info_data;

        Ok(())
    }
}
