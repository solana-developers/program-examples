use anchor_lang::prelude::*;
declare_id!("3cvZMR8oDVXVcxcfuPmBpsEWnGMYh2uomwYohNSJSWwk");
#[program]
pub mod account_data {
    use super::*;
    pub fn create_address_info(
        ctx: Context<CreateAddressInfoContext>,
        name: Vec<u8>,
        house_number: u8,
    ) -> Result<()> {
        ctx.accounts.address_info.house_number = house_number;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateAddressInfoContext<'info> {
    #[account(mut)]
    pub address_info: Account<'info, AddressInfo>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AddressInfo {
    pub name: Uint8Array,
    pub house_number: u8,
    pub street: String,
    pub city: String,
}
