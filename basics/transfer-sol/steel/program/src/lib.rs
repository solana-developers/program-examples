use steel::*;
use bytemuck::{Pod, Zeroable};
use num_enum::{IntoPrimitive, TryFromPrimitive};

declare_id!("11111111111111111111111111111111");

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum TransferInstructionType {
    CpiTransfer = 0,
    ProgramTransfer = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferArgs {
    pub amount: u64,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction::<TransferInstructionType>(&crate::ID, program_id, data)?;

    match ix {
        TransferInstructionType::CpiTransfer => process_cpi_transfer(accounts, data)?,
        TransferInstructionType::ProgramTransfer => process_program_transfer(accounts, data)?,
    }

    Ok(())
}

fn process_cpi_transfer(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Parse accounts
    let [payer_info, recipient_info, system_program_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    payer_info.is_signer()?.is_writable()?;
    recipient_info.is_writable()?;
    system_program_info.is_program(&system_program::ID)?;

    // Parse instruction data
    let args = bytemuck::try_from_bytes::<TransferArgs>(data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Transfer via CPI to system program
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            payer_info.key,
            recipient_info.key,
            args.amount,
        ),
        &[payer_info.clone(), recipient_info.clone()],
    )?;

    Ok(())
}

fn process_program_transfer(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Parse accounts
    let [payer_info, recipient_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    payer_info.is_signer()?.is_writable()?;
    recipient_info.is_writable()?;

    // Parse instruction data
    let args = bytemuck::try_from_bytes::<TransferArgs>(data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Transfer lamports directly
    **payer_info.try_borrow_mut_lamports()? -= args.amount;
    **recipient_info.try_borrow_mut_lamports()? += args.amount;

    Ok(())
}

entrypoint!(process_instruction);
