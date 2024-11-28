use crate::state::*;
use steel::*;
use sysvar::rent::Rent;

use super::SteelInstruction;

instruction!(SteelInstruction, ZeroInit);
/// work info
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct ZeroInit {
    pub work_info: WorkInfo,
}

impl ZeroInit {
    pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        // Steel uses zero_copy types, so the sizes are fixed, however we can move from one account
        // type to another, e.g ExtendedAddressInfo -> WorkInfo

        let [payer, address_info_account, _system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // we collect the extended data
        //
        let work_info_data = Self::try_from_bytes(data)?.work_info;

        // We get pay for the extra space we need before allocating the space
        //
        let account_span = 8 + std::mem::size_of::<WorkInfo>(); // 8 byte Account Discriminator + sizeof WorkInfo
        let lamports_required = (Rent::get()?).minimum_balance(account_span);
        let diff = lamports_required.saturating_sub(address_info_account.lamports());
        address_info_account.collect(diff, payer)?;

        // we reallocate new space to accomodate the new data
        //
        address_info_account.realloc(account_span, true)?; // zero init

        // we set the discriminator to the `WorkInfo`, so Steel can deserialize the account as such.
        //
        {
            let mut data = address_info_account.data.borrow_mut();
            data[0] = WorkInfo::discriminator();
        }

        // now we reset the account discriminator, we can deserialise as `WorkInfo``
        //
        let work_info = address_info_account.as_account_mut::<WorkInfo>(&crate::ID)?;

        // set the extended address info
        //
        *work_info = work_info_data;

        Ok(())
    }
}
