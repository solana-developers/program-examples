use anchor_lang::prelude::*;

use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateUserArgs {
    pub name: String,
}

#[derive(Accounts)]
#[instruction(args: CreateUserArgs)]
pub struct CreateUserContext<'info> {
    #[account(
        init,
        space = User::SIZE,
        payer = payer,
        seeds = [
            User::PREFIX.as_bytes(),
            payer.key().as_ref(),
        ],
        bump
    )]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_user(ctx: Context<CreateUserContext>, args: CreateUserArgs) -> Result<()> {
    let payer = &ctx.accounts.payer;
    let user_account = &mut ctx.accounts.user_account;

    msg!("{:#?}", ctx.bumps);

    user_account.bump = *ctx.bumps.get("user_account").expect("Bump not found.");
    user_account.user = payer.key();
    user_account.name = args.name;

    Ok(())
}
