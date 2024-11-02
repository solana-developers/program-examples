use anchor_lang::prelude::*;

declare_id!("8R1pBZKFyvBdR7LDa4R45JWSdUFnJdRSo9P1MPr571LC");

#[program]
pub mod pda_rent_payer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
