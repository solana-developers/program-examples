use anchor_lang::prelude::*;
declare_id!("DaNK9CdncCbPrHRWJpWL1oyEBS9M985YYXR8WTQzYSdE");
#[program]
pub mod hello_solana {
    use super::*;
    pub fn hello(ctx: Context<HelloContext>) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct HelloContext {}
