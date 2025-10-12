use crate::{require, states::page_visit::PageVisits};
use pinocchio::{
    account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

pub fn process_increament_page_visits(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    msg!("PageVisits Instruction: IncrementPageVisits");
    if let [user, creator, page_pda, _system_program] = accounts {
        require(
            user.is_signer() && user.is_writable(),
            ProgramError::MissingRequiredSignature,
            None,
        )?;

        let page_visit_data = PageVisits::check_id_with_bump(page_pda, creator.key())?;

        page_visit_data.page_visits += 1;

        Ok(())
    } else {
        Err(ProgramError::NotEnoughAccountKeys)
    }
}
