use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::{
    token_2022::{
        initialize_account3,
        spl_token_2022::{extension::ExtensionType, pod::PodAccount},
        InitializeAccount3,
    },
    token_interface::{
        memo_transfer_disable, memo_transfer_initialize, MemoTransfer, Mint, Token2022,
        TokenAccount,
    },
};

declare_id!("5BQyC7y2Pc283woThq11uZRqsgcRbBRLKz4yQ8BJadi2");

#[program]
pub mod memo_transfer {
    use super::*;

    pub fn initialize(context: Context<InitializeAccountConstraints>) -> Result<()> {
        // Calculate space required for token and extension data
        let token_account_size =
            ExtensionType::try_calculate_account_len::<PodAccount>(&[ExtensionType::MemoTransfer])?;

        // Calculate minimum lamports required for size of token account with extensions
        let lamports = (Rent::get()?).minimum_balance(token_account_size);

        // Invoke System Program to create new account with space for token account and extension data
        create_account(
            CpiContext::new(
                context.accounts.system_program.key(),
                CreateAccount {
                    from: context.accounts.payer.to_account_info(),
                    to: context.accounts.token_account.to_account_info(),
                },
            ),
            lamports,                          // Lamports
            token_account_size as u64,         // Space
            &context.accounts.token_program.key(), // Owner Program
        )?;

        // Initialize the standard token account data
        initialize_account3(CpiContext::new(
            context.accounts.token_program.key(),
            InitializeAccount3 {
                account: context.accounts.token_account.to_account_info(),
                mint: context.accounts.mint_account.to_account_info(),
                authority: context.accounts.payer.to_account_info(),
            },
        ))?;

        // Initialize the memo transfer extension
        // This instruction must come after the token account initialization
        memo_transfer_initialize(CpiContext::new(
            context.accounts.token_program.key(),
            MemoTransfer {
                token_program_id: context.accounts.token_program.to_account_info(),
                account: context.accounts.token_account.to_account_info(),
                owner: context.accounts.payer.to_account_info(),
            },
        ))?;
        Ok(())
    }

    pub fn disable(context: Context<DisableAccountConstraints>) -> Result<()> {
        memo_transfer_disable(CpiContext::new(
            context.accounts.token_program.key(),
            MemoTransfer {
                token_program_id: context.accounts.token_program.to_account_info(),
                account: context.accounts.token_account.to_account_info(),
                owner: context.accounts.owner.to_account_info(),
            },
        ))?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAccountConstraints<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub token_account: Signer<'info>,
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DisableAccountConstraints<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        token::authority = owner,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
}
