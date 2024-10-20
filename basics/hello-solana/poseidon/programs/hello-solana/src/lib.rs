use anchor_lang::prelude::*;
use poseidon::prelude::*;
declare_id!("DaNK9CdncCbPrHRWJpWL1oyEBS9M985YYXR8WTQzYSdE");

#[program]
pub mod hello_solana_program {
    use super::*;
    pub fn hello_solana(ctx: Context<HelloSolanaContext>) -> Result<()> {
        msg!("Hello, Solana!");
        Ok(())
    }
}
#[derive(Accounts)]
pub struct HelloSolanaContext {}