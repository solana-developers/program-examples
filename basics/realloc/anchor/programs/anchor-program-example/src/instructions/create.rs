use anchor_lang::prelude::*;

use crate::state::AddressInfo;


pub fn create_address_info(
    ctx: Context<Create>,
    name: String,
    house_number: u8,
    street: String,
    city: String,
) -> Result<()> {
    
    let address_info = AddressInfo::new(
        name,
        house_number,
        street,
        city,
    );
    ctx.accounts.target_account.set_inner(address_info.clone());
    Ok(())
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        init,
        payer = payer,
        space = AddressInfo::ACCOUNT_SPACE,
    )]
    pub target_account: Account<'info, AddressInfo>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
