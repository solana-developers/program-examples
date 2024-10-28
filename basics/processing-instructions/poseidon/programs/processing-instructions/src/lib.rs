use anchor_lang::prelude::*;
declare_id!("AthFDPn5w6LJLhez7ya4dcokjjZGLPozCPn3RUJrCkZ8");
#[program]
pub mod processing_instructions {
    use super::*;
    pub fn go_to_park(
        ctx: Context<GoToParkContext>,
        name: String,
        height: u32,
    ) -> Result<()> {
        ctx.accounts.user.name = name;
        ctx.accounts.user.height = height;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct GoToParkContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = 41, seeds = [payer.key().as_ref()], bump)]
    pub user: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct UserAccount {
    pub name: String,
    pub height: u32,
}
