use anchor_lang::prelude::*;
declare_id!("3PUaDfRezKNY9u2ffsAwgApxM3QYjztfYYcyNcuRKWmk");
#[program]
pub mod account_data {
    use super::*;
    pub fn create_address_info(
        ctx: Context<CreateAddressInfoContext>,
        house_number: u8,
        street_number: u8,
        city_zip_code: u32,
        name: String,
    ) -> Result<()> {
        ctx.accounts.address_info.name = name;
        ctx.accounts.address_info.house_number = house_number;
        ctx.accounts.address_info.street_number = street_number;
        ctx.accounts.address_info.city_zip_code = city_zip_code;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateAddressInfoContext<'info> {
    #[account(init, payer = payer, space = 39, seeds = [payer.key().as_ref()], bump)]
    pub address_info: Account<'info, AddressInfoState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AddressInfoState {
    pub house_number: u8,
    pub name: String,
    pub street_number: u8,
    pub city_zip_code: u32,
}
