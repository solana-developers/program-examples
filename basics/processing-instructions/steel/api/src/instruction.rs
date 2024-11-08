use solana_program::msg;
use std::ffi::CStr;
use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Zeroable, TryFromPrimitive, IntoPrimitive)]
pub enum ParkInstruction {
    Park = 0,
}

instruction!(ParkInstruction, Park);
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Park {
    pub name: [u8; 32],
    pub height: u32,
}

impl Park {
    pub fn process_instruction_data(instruction_data: &[u8]) -> ProgramResult {
        // Steel uses bytemuck under the hood to process instruction data
        //  bytemuck::try_from_bytes::<Park>(instruction_data)
        //
        let instruction_data_object = Park::try_from_bytes(instruction_data)?;

        msg!(
            "Welcome to the park, {:?}!",
            CStr::from_bytes_until_nul(&instruction_data_object.name).unwrap()
        );

        if instruction_data_object.height > 5 {
            msg!("You are tall enough to ride this ride. Congratulations.");
        } else {
            msg!("You are NOT tall enough to ride this ride. Sorry mate.");
        };

        Ok(())
    }
}
