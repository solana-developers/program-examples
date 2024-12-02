use cross_program_invocation_steel_lever_api::prelude::*;
use solana_program::program::invoke;
use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum HandInstruction {
    PullLever = 0,
}

instruction!(HandInstruction, PullLever);
/// PullLever Instruction
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PullLever {}

impl PullLever {
    pub fn process(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
        let [power, lever_program] = accounts else {
            return Err(ProgramError::InvalidInstructionData);
        };

        let set_power_status_data = SetPowerStatus::try_from_bytes(data)?;

        let ix = Instruction::new_with_bytes(
            *lever_program.key,                        // program id of the callee
            &set_power_status_data.to_bytes(),         // the instuction data,
            vec![AccountMeta::new(*power.key, false)], // accounts needed to execute the instruction
        );

        invoke(&ix, &[power.clone()])
    }
}
