pub mod instructions;
pub use instructions::*;

use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(program_id, program_id, data)?;

    match ix {
        SteelInstruction::InitializeLever => InitializeLever::process(program_id, accounts),
        SteelInstruction::SetPowerStatus => SetPowerStatus::process(program_id, accounts, data),
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    PowerStatus = 0,
}

account!(SteelAccount, PowerStatus);
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct PowerStatus {
    pub status: u8,
}

impl PowerStatus {
    pub fn switch(&mut self) -> ProgramResult {
        match self.status {
            0 => {
                // change the status to `1`
                self.status = 1;
                Ok(())
            }
            1 => {
                // change the status to `0`
                self.status = 0;
                Ok(())
            }
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
    pub fn is_on(&self) -> Result<bool, ProgramError> {
        match self.status {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
}
