use anchor_lang::prelude::*;
declare_id!("9zexDtgqhQvcMkCrZDco2oP4B5cyhmxMttt8aN52g6CP");
#[program]
pub mod hello_solana {
    use super::*;
    pub fn hello(ctx: Context<HelloContext>) -> Result<()> {
        msg!("Hello, Solana!");

        msg!("Our program's Program ID: {}", &id());

        Ok(())
    }
}
#[derive(Accounts)]
pub struct HelloContext {}
