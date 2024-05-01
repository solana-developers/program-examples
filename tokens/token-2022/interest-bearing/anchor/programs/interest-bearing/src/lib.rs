use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::{
    token_2022::{
        initialize_mint2,
        spl_token_2022::{
            extension::{
                interest_bearing_mint::InterestBearingConfig, BaseStateWithExtensions,
                ExtensionType, StateWithExtensions,
            },
            pod::PodMint,
            state::Mint as MintState,
        },
        InitializeMint2,
    },
    token_interface::{
        interest_bearing_mint_initialize, interest_bearing_mint_update_rate,
        spl_pod::optional_keys::OptionalNonZeroPubkey, InterestBearingMintInitialize,
        InterestBearingMintUpdateRate, Mint, Token2022,
    },
};
declare_id!("DMQdkzRJz8uQSN8Kx2QYmQJn6xLKhsu3LcPYxs314MgC");

#[program]
pub mod interest_bearing {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, rate: i16) -> Result<()> {
        // Calculate space required for mint and extension data
        let mint_size = ExtensionType::try_calculate_account_len::<PodMint>(&[
            ExtensionType::InterestBearingConfig,
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

        // Initialize the InterestBearingConfig extension
        // This instruction must come before the instruction to initialize the mint data
        interest_bearing_mint_initialize(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InterestBearingMintInitialize {
                    token_program_id: ctx.accounts.token_program.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                },
            ),
            Some(ctx.accounts.payer.key()),
            rate,
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

        check_mint_data(
            &ctx.accounts.mint_account.to_account_info(),
            &ctx.accounts.payer.key(),
        )?;
        Ok(())
    }

    pub fn update_rate(ctx: Context<UpdateRate>, rate: i16) -> Result<()> {
        interest_bearing_mint_update_rate(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InterestBearingMintUpdateRate {
                    token_program_id: ctx.accounts.token_program.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    rate_authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            rate,
        )?;

        check_mint_data(
            &ctx.accounts.mint_account.to_account_info(),
            &ctx.accounts.authority.key(),
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
pub struct UpdateRate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

fn check_mint_data(mint_account_info: &AccountInfo, authority_key: &Pubkey) -> Result<()> {
    let mint_data = mint_account_info.data.borrow();
    let mint_with_extension = StateWithExtensions::<MintState>::unpack(&mint_data)?;
    let extension_data = mint_with_extension.get_extension::<InterestBearingConfig>()?;

    assert_eq!(
        extension_data.rate_authority,
        OptionalNonZeroPubkey::try_from(Some(*authority_key))?
    );

    msg!("{:?}", extension_data);
    Ok(())
}
