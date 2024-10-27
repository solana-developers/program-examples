use anchor_lang::prelude::*;
declare_id!("CWy5sbubCYKdQ2ANmFmeZVRqxPJjE5NJ7S4SQBWHnPyF");
#[program]
pub mod account_data_program {
    use super::*;
    pub fn create_address_info(
        ctx: Context<CreateAddressInfoContext>,
        house_number: u8,
        street: u8,
        city_code: u32,
        name: String,
    ) -> Result<()> {
        ctx.accounts.state.name = name;
        ctx.accounts.state.house_number = house_number;
        ctx.accounts.state.street = street;
        ctx.accounts.state.city_code = city_code;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateAddressInfoContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 36,
        seeds = [b"address_info",
        payer.key().as_ref()],
        bump,
    )]
    pub state: Account<'info, AddressInfo>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AddressInfo {
    pub name: String,
    pub house_number: u8,
    pub street: u8,
    pub city_code: u32,
}
