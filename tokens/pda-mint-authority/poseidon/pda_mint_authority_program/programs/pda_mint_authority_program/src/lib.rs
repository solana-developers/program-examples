use anchor_lang::prelude::*;

declare_id!("2dR87YancKtdVJSxfyrYGRCGd7GqGCsC2v1RutTC6ozz");

#[program]
pub mod pda_mint_authority_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
