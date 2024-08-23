use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump,
        space = UserAccount::INIT_SPACE,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    
    pub fn initialize_user(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount { 
            points: 0, 
            amount_staked: 0, 
            bump: bumps.user_account 
        });

        Ok(())
    }
}