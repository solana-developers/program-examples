use anchor_lang::prelude::*;
declare_id!("ChA1o71vBEwkYNs6FnkmG4cxyZWtWkbXSEJ6xP2zaJAq");
#[program]
pub mod address_info_program {
    use super::*;
    pub fn initialize(
        ctx: Context<InitializeContext>,
        house_number: u8,
        street_number: u16,
        zip_code: u32,
        country_code: u16,
    ) -> Result<()> {
        ctx.accounts.state.owner = ctx.accounts.owner.key();
        ctx.accounts.state.house_number = house_number;
        ctx.accounts.state.street_number = street_number;
        ctx.accounts.state.zip_code = zip_code;
        ctx.accounts.state.country_code = country_code;
        ctx.accounts.state.bump = ctx.bumps.state;
        Ok(())
    }
    pub fn edit(
        ctx: Context<EditContext>,
        house_number: u8,
        street_number: u16,
        zip_code: u32,
        country_code: u16,
    ) -> Result<()> {
        ctx.accounts.state.house_number = house_number;
        ctx.accounts.state.street_number = street_number;
        ctx.accounts.state.zip_code = zip_code;
        ctx.accounts.state.country_code = country_code;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(
        init,
        payer = owner,
        space = 50,
        seeds = [b"address_info",
        owner.key().as_ref()],
        bump,
    )]
    pub state: Account<'info, AddressInfoState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct EditContext<'info> {
    #[account(mut, seeds = [b"address_info", owner.key().as_ref()], bump)]
    pub state: Account<'info, AddressInfoState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AddressInfoState {
    pub owner: Pubkey,
    pub bump: u8,
    pub house_number: u8,
    pub street_number: u16,
    pub zip_code: u32,
    pub country_code: u16,
}
