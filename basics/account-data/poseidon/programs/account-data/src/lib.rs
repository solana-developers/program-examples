use anchor_lang::prelude::*;
declare_id!("3PUaDfRezKNY9u2ffsAwgApxM3QYjztfYYcyNcuRKWmk");
#[program]
pub mod account_data {
    use super::*;
    pub fn create_address_info(
        ctx: Context<CreateAddressInfoContext>,
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Result<()> {
        ctx.accounts.address_info.name = name;
        ctx.accounts.address_info.house_number = house_number;
        ctx.accounts.address_info.street = street;
        ctx.accounts.address_info.city = city;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateAddressInfoContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = 96, seeds = [payer.key().as_ref()], bump)]
    pub address_info: Account<'info, AddressInfoState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AddressInfoState {
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
}
