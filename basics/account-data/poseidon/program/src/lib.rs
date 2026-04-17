use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

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
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + AddressInfo::INIT_SPACE,
        seeds = [b"address_info", owner.key().as_ref()],
        bump
    )]
    pub address_info: Account<'info, AddressInfo>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct AddressInfo {
    #[max_len(32)]
    pub name: String,
    pub house_number: u8,
    #[max_len(32)]
    pub street: String,
    #[max_len(32)]
    pub city: String,
}
