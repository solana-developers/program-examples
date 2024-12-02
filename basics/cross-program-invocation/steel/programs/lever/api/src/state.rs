use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum LeverAccount {
    PowerStatus = 0,
}

account!(LeverAccount, PowerStatus);
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct PowerStatus {
    pub on: u8,
}

impl PowerStatus {
    pub fn switch(&mut self) -> ProgramResult {
        match self.on {
            0 => {
                // change the status to `1`
                self.on = 1;
                Ok(())
            }
            1 => {
                // change the status to `0`
                self.on = 0;
                Ok(())
            }
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
    pub fn is_on(&self) -> Result<bool, ProgramError> {
        match self.on {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
}
