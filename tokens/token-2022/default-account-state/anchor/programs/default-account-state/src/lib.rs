use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::{
    token_2022::{
        initialize_mint2,
        spl_token_2022::{extension::ExtensionType, pod::PodMint, state::AccountState},
        InitializeMint2,
    },
    token_interface::{
        default_account_state_initialize, default_account_state_update,
        DefaultAccountStateInitialize, DefaultAccountStateUpdate, Mint, Token2022,
    },
};

declare_id!("5LdYbHiUsFxVG8bfqoeBkhBYMRmWZb3BoLuABgYW7coB");

#[program]
pub mod default_account_state {
    use super::*;

    // There is currently not an anchor constraint to automatically initialize the DefaultAccountState extension
    // We can manually create and initialize the mint account via CPIs in the instruction handler
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Calculate space required for mint and extension data
        let mint_size = ExtensionType::try_calculate_account_len::<PodMint>(&[
            ExtensionType::DefaultAccountState,
        ])?;

        // Calculate minimum lamports required for size of mint account with extensions
        let lamports = (Rent::get()?).minimum_balance(mint_size);

        // Invoke System Program to create new account with space for mint and extension data
        create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.mint_account.to_account_info(),
                },
            ),
            lamports,                          // Lamports
            mint_size as u64,                  // Space
            &ctx.accounts.token_program.key(), // Owner Program
        )?;

        // Initialize the NonTransferable extension
        // This instruction must come before the instruction to initialize the mint data
        default_account_state_initialize(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                DefaultAccountStateInitialize {
                    token_program_id: ctx.accounts.token_program.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                },
            ),
            &AccountState::Frozen, // default frozen token accounts
        )?;

        // Initialize the standard mint account data
        initialize_mint2(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint2 {
                    mint: ctx.accounts.mint_account.to_account_info(),
                },
            ),
            2,                               // decimals
            &ctx.accounts.payer.key(),       // mint authority
            Some(&ctx.accounts.payer.key()), // freeze authority
        )?;
        Ok(())
    }

    pub fn update_default_state(
        ctx: Context<UpdateDefaultState>,
        account_state: AnchorAccountState,
    ) -> Result<()> {
        // Convert AnchorAccountState to spl_token_2022::state::AccountState
        let account_state = account_state.to_spl_account_state();

        default_account_state_update(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                DefaultAccountStateUpdate {
                    token_program_id: ctx.accounts.token_program.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    freeze_authority: ctx.accounts.freeze_authority.to_account_info(),
                },
            ),
            &account_state,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub mint_account: Signer<'info>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateDefaultState<'info> {
    #[account(mut)]
    pub freeze_authority: Signer<'info>,
    #[account(
        mut,
        mint::freeze_authority = freeze_authority,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

// Custom enum to implement AnchorSerialize and AnchorDeserialize
// This is required to pass the enum as an argument to the instruction
#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AnchorAccountState {
    Uninitialized,
    Initialized,
    Frozen,
}

// Implement conversion from AnchorAccountState to spl_token_2022::state::AccountState
impl AnchorAccountState {
    pub fn to_spl_account_state(&self) -> AccountState {
        match self {
            AnchorAccountState::Uninitialized => AccountState::Uninitialized,
            AnchorAccountState::Initialized => AccountState::Initialized,
            AnchorAccountState::Frozen => AccountState::Frozen,
        }
    }
}
