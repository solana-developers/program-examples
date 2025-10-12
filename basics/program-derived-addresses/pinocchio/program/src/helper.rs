use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, msg, program_error::ProgramError};
pub fn require(
    condition: bool,
    err: ProgramError,
    reason: Option<&str>,
) -> Result<(), ProgramError> {
    if !condition {
        if let Some(error) = reason {
            msg!(error)
        }
        Err(err)
    } else {
        Ok(())
    }
}

pub fn load<T>(account: &AccountInfo) -> Result<&mut T, ProgramError>
where
    T: Pod + Zeroable,
{
    let data = unsafe { account.borrow_mut_data_unchecked() };

    bytemuck::try_from_bytes_mut::<T>(data).map_err(|_| ProgramError::InvalidAccountData)
}
