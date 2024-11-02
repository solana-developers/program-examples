use cross_program_invocation_steel_lever::SetPowerStatus;
use solana_program::program::invoke;
use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
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
