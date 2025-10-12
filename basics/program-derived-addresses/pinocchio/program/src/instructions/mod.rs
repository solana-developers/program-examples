use pinocchio::program_error::ProgramError;

pub mod create_page;
pub mod increment_page_visits;

#[repr(u8)]
pub enum CreatePageInstructions {
    CreatePage,
    IncrementPageVisits,
}

impl TryFrom<&u8> for CreatePageInstructions {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(CreatePageInstructions::CreatePage),
            1 => Ok(CreatePageInstructions::IncrementPageVisits),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
