use pinocchio::program_error::ProgramError;
pub mod create_counter;
pub mod increment_counter;
#[repr(u8)]
pub enum CounterInstructions {
    CreateCounter,
    Increment,
}

impl TryFrom<&u8> for CounterInstructions {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(CounterInstructions::CreateCounter),
            1 => Ok(CounterInstructions::Increment),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
