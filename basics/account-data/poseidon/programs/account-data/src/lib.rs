use anchor_lang::prelude::*;
declare_id!("CFiSZ4w8WcG7U8Axq3Lx5zpbyLiMMBde6HGKtZvRCn6U");
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
    #[account(
        init,
        payer = owner,
        space = 171,
        seeds = [b"address_info", owner.key().as_ref()],
        bump,
    )]
    pub address_info: Account<'info, AddressInfo>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AddressInfo {
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
}
