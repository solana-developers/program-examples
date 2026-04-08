use anchor_lang::prelude::*;

declare_id!("Fod47xKXjdHVQDzkFPBvfdWLm8gEAV4iMSXkfUzCHiSD");

#[program]
pub mod anchor_realloc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input: String) -> Result<()> {
        ctx.accounts.message_account.message = input;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, input: String) -> Result<()> {
        ctx.accounts.message_account.message = input;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(input: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = Message::required_space(input.len()),
    )]
    pub message_account: Account<'info, Message>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(input: String)]
pub struct Update<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        realloc = Message::required_space(input.len()),
        realloc::payer = payer,
        realloc::zero = true,
    )]
    pub message_account: Account<'info, Message>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Message {
    pub message: String,
}

impl Message {
    pub fn required_space(input_len: usize) -> usize {
        8 + // 8 byte discriminator
        4 + // 4 byte for length of string
        input_len
    }
}
