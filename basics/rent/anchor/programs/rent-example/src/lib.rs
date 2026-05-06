use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("ED6f4gweAE7hWPQPXMt4kWxzDJne8VQEm9zkb1tMpFNB");

#[program]
pub mod rent_example {
    use super::*;

    pub fn create_system_account(
        context: Context<CreateSystemAccount>,
        address_data: AddressData,
    ) -> Result<()> {
        msg!("Program invoked. Creating a system account...");
        msg!(
            "  New public key will be: {}",
            &context.accounts.new_account.key().to_string()
        );

        // Determine the necessary minimum rent by calculating the account's size
        //
        // borsh 1.x: try_to_vec() removed, use borsh::to_vec() instead
        let account_span = anchor_lang::prelude::borsh::to_vec(&address_data)?.len();
        let lamports_required = (Rent::get()?).minimum_balance(account_span);

        msg!("Account span: {}", &account_span);
        msg!("Lamports required: {}", &lamports_required);

        system_program::create_account(
            CpiContext::new(
                context.accounts.system_program.key(),
                system_program::CreateAccount {
                    from: context.accounts.payer.to_account_info(),
                    to: context.accounts.new_account.to_account_info(),
                },
            ),
            lamports_required,
            account_span as u64,
            &context.accounts.system_program.key(),
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

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct AddressData {
    name: String,
    address: String,
}
