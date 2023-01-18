use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;


declare_id!("EDqtckbgeVKebuD86BBpBda1p6mZbE1Rpsa5jdTszQrg");


#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn create_address_info(
        ctx: Context<Create>,
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Result<()> {
        
        create::create_address_info(
            ctx,
            name,
            house_number,
            street,
            city,
        )
    }

    pub fn reallocate_without_zero_init(
        ctx: Context<ReallocateWithoutZeroInit>,
        state: String,
        zip: u32,
    ) -> Result<()> {
        
        reallocate::reallocate_without_zero_init(
            ctx,
            state,
            zip,
        )
    }

    pub fn reallocate_zero_init(
        ctx: Context<ReallocateZeroInit>,
        name: String,
        position: String,
        company: String,
        years_employed: u8,
    ) -> Result<()> {
        
        reallocate::reallocate_zero_init(
            ctx,
            name,
            position,
            company,
            years_employed,
        )
    }
}
