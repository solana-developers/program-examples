use solana_program::rent::Rent;
use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Parse data into address info.
    let address_info_data = bytemuck::try_from_bytes::<AddressInfo>(data)
        .or(Err(ProgramError::InvalidInstructionData))?;

    let [payer, address_info_account, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    payer.is_signer()?;
    address_info_account.is_signer()?;
    address_info_account.is_empty()?;
    system_program.is_program(&system_program::ID)?;

    let rent = Rent::get()?;
    let space = 8 + std::mem::size_of::<AddressInfo>();

    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            payer.key,
            address_info_account.key,
            rent.minimum_balance(space),
            space as u64,
            program_id,
        ),
        &[
            payer.clone(),
            address_info_account.clone(),
            system_program.clone(),
        ],
    )?;

    let address_info = address_info_account.as_account_mut::<AddressInfo>(program_id)?;

    *address_info = *address_info_data;

    Ok(())
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    AddressInfo = 0,
}

account!(SteelAccount, AddressInfo);
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AddressInfo {
    pub name: [u8; 48],
    pub house_number: u8,
    pub street: [u8; 48],
    pub city: [u8; 48],
}
