use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("oCCQRZyAbVxujyd8m57MPmDzZDmy2FoKW4ULS7KofCE");

#[program]
pub mod program_derived_addresses_program {
    use super::*;

    pub fn create_page_visits(context: Context<CreatePageVisitsAccountConstraints>) -> Result<()> {
        create::handle_create_page_visits(context)
    }

    pub fn increment_page_visits(context: Context<IncrementPageVisitsAccountConstraints>) -> Result<()> {
        increment::handle_increment_page_visits(context)
    }
}
