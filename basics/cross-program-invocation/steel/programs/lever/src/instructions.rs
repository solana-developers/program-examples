use std::ffi::CStr;

use solana_program::{msg, program::invoke, rent::Rent, system_instruction};
use steel::*;

use crate::PowerStatus;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    InitializeLever = 0,
    SetPowerStatus = 1,
}

instruction!(SteelInstruction, InitializeLever);
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InitializeLever {}

impl InitializeLever {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let [power, user, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        let account_span = 8 + std::mem::size_of::<PowerStatus>(); // 8 byte discriminator + account data

        let lamports_required = (Rent::get()?).minimum_balance(account_span);

        invoke(
            &system_instruction::create_account(
                user.key,
                power.key,
                lamports_required,
                account_span as u64,
                program_id,
            ),
            &[user.clone(), power.clone(), system_program.clone()],
        )
    }
}

instruction!(SteelInstruction, SetPowerStatus);
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetPowerStatus {
    pub name: [u8; 48],
}

impl SetPowerStatus {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
        let name = SetPowerStatus::try_from_bytes(data)?.name;

        let [power_info] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        let power = power_info.as_account_mut::<PowerStatus>(program_id)?;

        // switch power status
        power.switch()?;

        msg!(
            "{:?} is pulling the power switch!",
            CStr::from_bytes_until_nul(&name).unwrap()
        );

        match power.is_on()? {
            true => msg!("The power is now on."),
            false => msg!("The power is now off!"),
        };

        Ok(())
    }
}
