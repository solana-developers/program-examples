use anchor_lang::prelude::*;
declare_id!("BWj4tkT21WrGfyi1hcYKjAKrB3UbXm144hZ84RaMLV7C");
#[program]
pub mod processing_instructions {
    use super::*;
}
#[account]
pub struct GreetingAccount {
    pub last_updated: u64,
}
