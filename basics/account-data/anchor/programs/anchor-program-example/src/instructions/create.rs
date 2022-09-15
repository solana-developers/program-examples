use anchor_lang::prelude::*;

use crate::state::AddressInfo;


pub fn create_address_info(
    ctx: Context<CreateAddressInfo>,
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

    let account_span = (address_info.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    system_program::create_account(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::CreateAccount {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.address_info.to_account_info(),
            },
        ),
        lamports_required,
        account_span as u64,
        &ctx.accounts.system_program.key(),
    )?;

    let address_info_account = &mut ctx.accounts.address_info;
    address_info_account.set_inner(address_info);
    Ok(())
}

#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    address_info: Account<'info, AddressInfo>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}