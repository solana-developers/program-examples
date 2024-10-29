use anchor_lang::prelude::*;

declare_id!("DmrXSUGWYaqtWg8sbi9JQN48yVZ1y2m7HvWXbND52Mcw");

#[program]
pub mod spl_token_minter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
