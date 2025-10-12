use crate::{load, require, states::page_visit::PageVisits};
use {
    pinocchio::{
        account_info::AccountInfo,
        instruction::{Seed, Signer},
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvars::{rent::Rent, Sysvar},
        ProgramResult,
    },
    pinocchio_system::instructions::CreateAccount,
};

pub fn process_create_page(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("PageVisits Instruction: CreatePage");
    if let [creator, page_pda, _system_program] = accounts {
        require(
            creator.is_signer() && creator.is_writable(),
            ProgramError::MissingRequiredSignature,
            None,
        )?;

        let bump = PageVisits::check_id(page_pda.key(), creator.key())?;

        let seed_bumps = &[bump];
        let seeds = [
            Seed::from(PageVisits::PREFIX),
            Seed::from(creator.key().as_ref()),
            Seed::from(seed_bumps),
        ];

        let signer_seeds = Signer::from(&seeds);

        CreateAccount {
            from: creator,
            lamports: Rent::get()?.minimum_balance(PageVisits::SIZE),
            owner: program_id,
            space: PageVisits::SIZE as u64,
            to: page_pda,
        }
        .invoke_signed(&[signer_seeds])?;

        let page_data = load::<PageVisits>(page_pda)?;

        page_data.page_visits = 0;
        page_data.bump = bump;

        Ok(())
    } else {
        Err(ProgramError::NotEnoughAccountKeys)
    }
}
