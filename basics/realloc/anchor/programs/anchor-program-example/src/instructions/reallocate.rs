use anchor_lang::prelude::*;

use crate::state::{
    AddressInfo,
    EnhancedAddressInfo,
    WorkInfo,
};


pub fn reallocate_without_zero_init(
    ctx: Context<ReallocateWithoutZeroInit>,
    state: String,
    zip: u32,
) -> Result<()> {
    
    let enhanced_address_info = EnhancedAddressInfo::from_address_info(
        ctx.accounts.target_account.clone().into_inner(),
        state,
        zip,
    );
    ctx.accounts.target_account.set_inner(enhanced_address_info.clone());
    Ok(())
}

#[derive(Accounts)]
pub struct ReallocateWithoutZeroInit<'info> {
    #[account(
        mut,
        realloc = EnhancedAddressInfo::ACCOUNT_SPACE,
        realloc::payer = payer,
        realloc::zero = false,
    )]
    pub target_account: Account<'info, AddressInfo>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn reallocate_zero_init(
    ctx: Context<ReallocateZeroInit>,
    name: String,
    position: String,
    company: String,
    years_employed: u8,
) -> Result<()> {

    let work_info = WorkInfo::new(
        name,
        position,
        company,
        years_employed,
    );
    ctx.accounts.target_account.set_inner(work_info.clone());
    Ok(())
}

#[derive(Accounts)]
pub struct ReallocateZeroInit<'info> {
    #[account(
        mut,
        realloc = WorkInfo::ACCOUNT_SPACE,
        realloc::payer = payer,
        realloc::zero = true,
    )]
    pub target_account: Account<'info, WorkInfo>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}