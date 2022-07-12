use anchor_lang::prelude::*;
use anchor_lang::system_program;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


#[program]
pub mod rent_example {
    use super::*;

    pub fn create_system_account(ctx: Context<CreateSystemAccount>) -> Result<()> {

        msg!("Program invoked. Creating a system account...");
        msg!("  New public key will be: {}", &ctx.accounts.new_account.key().to_string());

        // Determine the necessary minimum rent by calculating the account's size
        //
        let account_span = amount as usize;
        let lamports_required = (Rent::get()?).minimum_balance(account_span);

        msg!("Account span: {}", &account_span);
        msg!("Lamports required: {}", &lamports_required);

        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.new_account.to_account_info(),
                },
            ),
            lamports_required,
            account_span,
            &ctx.accounts.system_program.key(),
        )?;

        msg!("Account created succesfully.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateSystemAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub new_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}