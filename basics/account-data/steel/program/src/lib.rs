use steel::*;
use bytemuck::{Pod, Zeroable};
use num_enum::{IntoPrimitive, TryFromPrimitive};

declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AddressInstructionType {
    Create = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct AddressInfo {
    pub name: [u8; 32],
    pub house_number: u8,
    pub street: [u8; 32],
    pub city: [u8; 32],
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction::<AddressInstructionType>(&crate::ID, program_id, data)?;

    match ix {
        AddressInstructionType::Create => process_create(program_id, accounts, data)?,
    }

    Ok(())
}

fn process_create(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Parse accounts
    let [address_info_account, payer_info, system_program_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    address_info_account.is_signer()?.is_writable()?.is_empty()?;
    payer_info.is_signer()?.is_writable()?;
    system_program_info.is_program(&system_program::ID)?;

    // Parse instruction data
    let address_info = bytemuck::try_from_bytes::<AddressInfo>(data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Calculate space and rent
    let space = 8 + std::mem::size_of::<AddressInfo>();
    let rent = solana_program::rent::Rent::get()?;
    let lamports = rent.minimum_balance(space);

    // Create account via CPI
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            payer_info.key,
            address_info_account.key,
            lamports,
            space as u64,
            program_id,
        ),
        &[
            payer_info.clone(),
            address_info_account.clone(),
            system_program_info.clone(),
        ],
    )?;

    // Write data to account
    let mut account_data = address_info_account.try_borrow_mut_data()?;
    account_data[0] = 0; // Discriminator
    account_data[8..8 + std::mem::size_of::<AddressInfo>()]
        .copy_from_slice(bytemuck::bytes_of(address_info));

    Ok(())
}

entrypoint!(process_instruction);
