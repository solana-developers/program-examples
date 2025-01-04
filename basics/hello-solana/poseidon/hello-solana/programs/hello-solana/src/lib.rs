use anchor_lang::prelude::*;

declare_id!("2phbC62wekpw95XuBk4i1KX4uA8zBUWmYbiTMhicSuBV");

#[program]
pub mod hello_solana_program {
    use super::*;
    pub fn hello(_ctx: Context<HelloContext>) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct HelloContext {}
