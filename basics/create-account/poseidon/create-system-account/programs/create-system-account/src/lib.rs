use anchor_lang::prelude::*;

declare_id!("HiodPTcV4ZBV8GkqNPRhJKuVoBAxzEQYxK2Mbv9i9vY4");

#[program]
pub mod create_system_account_program {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        ctx.accounts.state.owner = ctx.accounts.owner.key();
        ctx.accounts.state.auth_bump = ctx.bumps.auth;
        ctx.accounts.state.account_bump = ctx.bumps.state;
        Ok(())
    }
    pub fn update(ctx: Context<UpdateContext>, new_value: u8) -> Result<()> {
        ctx.accounts.state.value = new_value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account()]
    /// CHECK: This acc is safe
    pub auth: UncheckedAccount<'info>,
    #[account(init, payer = owner, space = 43, seeds = [b"account"], bump)]
    pub state: Account<'info, AccountState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateContext<'info> {
    #[account(mut, seeds = [b"account"], bump)]
    pub state: Account<'info, AccountState>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AccountState {
    pub owner: Pubkey,
    pub value: u8,
    pub account_bump: u8,
    pub auth_bump: u8,
}
