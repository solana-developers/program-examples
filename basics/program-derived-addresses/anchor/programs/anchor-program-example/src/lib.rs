#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("oCCQRZyAbVxujyd8m57MPmDzZDmy2FoKW4ULS7KofCE");

#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn create_page_visits(ctx: Context<CreatePageVisits>) -> Result<()> {
        create::create_page_visits(ctx)
    }

    pub fn increment_page_visits(ctx: Context<IncrementPageVisits>) -> Result<()> {
        increment::increment_page_visits(ctx)
    }
}
