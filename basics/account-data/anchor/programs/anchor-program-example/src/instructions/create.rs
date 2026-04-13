use crate::{constants::ANCHOR_DISCRIMINATOR_SIZE, state::AddressInfo};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = ANCHOR_DISCRIMINATOR_SIZE + AddressInfo::INIT_SPACE,
    )]
    address_info: Account<'info, AddressInfo>,
    system_program: Program<'info, System>,
}

pub fn handle_create_address_info(
    context: Context<CreateAddressInfo>,
    name: String,
    house_number: u8,
    street: String,
    city: String,
) -> Result<()> {
    *context.accounts.address_info = AddressInfo {
        name,
        house_number,
        street,
        city,
    };
    Ok(())
}
