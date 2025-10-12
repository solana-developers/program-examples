use bytemuck::{Pod, Zeroable};
use {
    pinocchio::{
        account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
    },
    pinocchio_system::instructions::Transfer,
};
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct CpiTransferArgs {
    amount: u64,
}

pub fn process_cpi_transfer(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    ix_data: &[u8],
) -> ProgramResult {
    msg!("TransferSol Instruction: CpiTransfer");
    if let [sender, receiver, _system_program] = accounts {
        let mut aligned_ix_buf = [0u8; core::mem::size_of::<CpiTransferArgs>()]; // putting raw ix_data will fail since it started at index 1 of the original instruction_data, so this new allocation is required

        aligned_ix_buf.copy_from_slice(ix_data);

        let params = bytemuck::try_from_bytes::<CpiTransferArgs>(&aligned_ix_buf)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        // sol_log_64(params.amount, 0, 0, 0, 0);

        Transfer {
            from: sender,
            lamports: params.amount,
            to: receiver,
        }
        .invoke()?;
    }

    Ok(())
}
