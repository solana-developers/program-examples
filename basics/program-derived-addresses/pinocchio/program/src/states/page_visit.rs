use crate::{load, require};
use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{create_program_address, find_program_address, pubkey_eq, Pubkey},
};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct PageVisits {
    pub bump: u8,
    pub _padding: [u8; 7],
    pub page_visits: u64,
}

impl PageVisits {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub const PREFIX: &[u8] = b"page_visits";

    pub fn check_id(page_pda: &Pubkey, creator: &Pubkey) -> Result<u8, ProgramError> {
        let seeds = [Self::PREFIX, creator.as_ref()];

        let (expected_page_id, bump) = find_program_address(&seeds, &crate::ID);

        require(
            pubkey_eq(&expected_page_id, page_pda),
            ProgramError::IncorrectProgramId,
            Some("Validation Error: Incorrect Page Pda address"),
        )?;

        Ok(bump)
    }

    pub fn check_id_with_bump<'a>(
        page_pda: &'a AccountInfo,
        creator: &'a Pubkey,
    ) -> Result<&'a mut Self, ProgramError> {
        let page_pda_data = load::<Self>(page_pda)?;

        let seeds = [Self::PREFIX, creator.as_ref(), &[page_pda_data.bump]];

        let expected_pda_address = create_program_address(&seeds, &crate::ID)?;

        require(
            pubkey_eq(&expected_pda_address, page_pda.key()),
            ProgramError::IncorrectProgramId,
            Some("Validation Error: invalid page pda address"),
        )?;

        Ok(page_pda_data)
    }
}
