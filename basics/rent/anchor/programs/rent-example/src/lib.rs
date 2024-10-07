#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("ED6f4gweAE7hWPQPXMt4kWxzDJne8VQEm9zkb1tMpFNB");

#[program]
pub mod rent_example {
    use super::*;

    pub fn create_system_account(
        ctx: Context<CreateSystemAccount>,
        address_data: AddressData,
    ) -> Result<()> {
        msg!("Program invoked. Creating a system account...");

        // Determine the necessary minimum rent based on the account size
        let account_data_size = address_data.try_to_vec()?.len();
        let lamports_required = Rent::get()
            .map_err(|_| RentError::RentCalculationFailed)?
            .minimum_balance(account_data_size);

        if **ctx.accounts.payer.try_borrow_lamports()? < lamports_required {
            return Err(RentError::InsufficientLamports.into());
        }

        msg!("Account size: {} bytes", account_data_size);
        msg!(
            "Lamports required for rent exemption: {}",
            lamports_required
        );

        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.new_account.to_account_info(),
                },
            ),
            lamports_required,
            account_data_size as u64,
            &ctx.accounts.system_program.key(),
        )?;

        msg!(
            "Account successfully created with public key: {}",
            ctx.accounts.new_account.key()
        );

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

#[error_code]
pub enum RentError {
    #[msg("Failed to calculate rent required.")]
    RentCalculationFailed,
    #[msg("Account creation failed due to insufficient lamports.")]
    InsufficientLamports,
}
