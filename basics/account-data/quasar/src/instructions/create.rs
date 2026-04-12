use {
    crate::state::AddressInfo,
    quasar_lang::prelude::*,
};

/// Accounts for creating a new address info account.
/// Dynamic accounts use owned `Account<T>` rather than `&'info mut Account<T>` because
/// dynamic types carry cached byte offsets that cannot be represented as a pointer cast.
#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    pub payer: &'info mut Signer,
    #[account(mut, init, payer = payer, seeds = [b"address_info", payer], bump)]
    pub address_info: Account<AddressInfo<'info>>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_create_address_info(
    accounts: &mut CreateAddressInfo, name: &str,
    house_number: u8,
    street: &str,
    city: &str,
) -> Result<(), ProgramError> {
    accounts.address_info.set_inner(
        house_number,
        name,
        street,
        city,
        accounts.payer.to_account_view(),
        None,
    )
}
