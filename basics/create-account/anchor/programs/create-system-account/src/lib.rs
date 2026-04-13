use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};

declare_id!("ARVNCsYKDQsCLHbwUTJLpFXVrJdjhWZStyzvxmKe2xHi");

#[program]
pub mod create_system_account {
    use super::*;

    pub fn create_system_account(context: Context<CreateSystemAccount>) -> Result<()> {
        msg!("Program invoked. Creating a system account...");
        msg!(
            "  New public key will be: {}",
            &context.accounts.new_account.key().to_string()
        );

        // The minimum lamports for rent exemption
        let lamports = (Rent::get()?).minimum_balance(0);

        create_account(
            CpiContext::new(
                context.accounts.system_program.key(),
                CreateAccount {
                    from: context.accounts.payer.to_account_info(), // From pubkey
                    to: context.accounts.new_account.to_account_info(), // To pubkey
                },
            ),
            lamports,                           // Lamports
            0,                                  // Space
            &context.accounts.system_program.key(), // Owner Program
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
