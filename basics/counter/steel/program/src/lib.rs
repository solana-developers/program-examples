mod state;

use solana_program::msg;
use state::*;
use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (instruction_discriminant, instruction_data_inner) = instruction_data.split_at(1);
    
    match instruction_discriminant[0] {
        0 => {
            msg!("Instruction: Increment");
            process_increment_counter(program_id, accounts, instruction_data_inner)?;
        }
        _ => {
            msg!("Error: unknown instruction")
        }
    }
    Ok(())
}

pub fn process_increment_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let [counter_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    counter_info.is_writable()?;

    let counter = counter_info.as_account_mut::<Counter>(program_id)?;

    counter.count += 1;

    let count = counter.count;

    msg!("Counter state incremented to {:?}", count);
    Ok(())
}
