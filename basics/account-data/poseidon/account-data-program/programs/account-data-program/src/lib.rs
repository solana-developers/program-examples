use anchor_lang::prelude::*;
declare_id!("JvF1QDhgab1ARhACPWTAZnUymthGGmn3NXCj8i6mjSQ");
#[program]
pub mod account_data_program {
    use super::*;
    pub fn create_address_info(
        ctx: Context<CreateAddressInfoContext>,
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Result<()> {
        ctx.accounts.state.name = name;
        ctx.accounts.state.house_number = house_number;
        ctx.accounts.state.street = street;
        ctx.accounts.state.city = city;
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
        space = 171,
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
    pub street: String,
    pub city: String,
}
