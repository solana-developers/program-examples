use crate::state::AddressInfo;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + AddressInfo::INIT_SPACE,
    )]
    address_info: Account<'info, AddressInfo>,
    system_program: Program<'info, System>,
}

pub fn create_address_info(
    ctx: Context<CreateAddressInfo>,
    name: String,
    house_number: u8,
    street: String,
    city: String,
) -> Result<()> {
    *ctx.accounts.address_info = AddressInfo {
        name,
        house_number,
        street,
        city,
    };
    Ok(())
}
