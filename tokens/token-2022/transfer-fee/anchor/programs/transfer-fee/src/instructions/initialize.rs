use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::{
    token_2022::{
        initialize_mint2,
        spl_token_2022::{
            extension::{
                transfer_fee::TransferFeeConfig, BaseStateWithExtensions, ExtensionType,
                StateWithExtensions,
            },
            pod::PodMint,
            state::Mint as MintState,
        },
        InitializeMint2,
    },
    token_interface::{
        spl_pod::optional_keys::OptionalNonZeroPubkey, transfer_fee_initialize, Token2022,
        TransferFeeInitialize,
    },
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub mint_account: Signer<'info>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

// There is currently not an anchor constraint to automatically initialize the TransferFeeConfig extension
// We can manually create and initialize the mint account via CPIs in the instruction handler
pub fn handle_process_initialize(
    context: Context<Initialize>,
    transfer_fee_basis_points: u16,
    maximum_fee: u64,
) -> Result<()> {
    // Calculate space required for mint and extension data
    let mint_size =
        ExtensionType::try_calculate_account_len::<PodMint>(&[ExtensionType::TransferFeeConfig])?;

    // Calculate minimum lamports required for size of mint account with extensions
    let lamports = (Rent::get()?).minimum_balance(mint_size);

    // Invoke System Program to create new account with space for mint and extension data
    create_account(
        CpiContext::new(
            context.accounts.system_program.key(),
            CreateAccount {
                from: context.accounts.payer.to_account_info(),
                to: context.accounts.mint_account.to_account_info(),
            },
        ),
        lamports,                          // Lamports
        mint_size as u64,                  // Space
        &context.accounts.token_program.key(), // Owner Program
    )?;

    // Initialize the transfer fee extension data
    // This instruction must come before the instruction to initialize the mint data
    transfer_fee_initialize(
        CpiContext::new(
            context.accounts.token_program.key(),
            TransferFeeInitialize {
                token_program_id: context.accounts.token_program.to_account_info(),
                mint: context.accounts.mint_account.to_account_info(),
            },
        ),
        Some(&context.accounts.payer.key()), // transfer fee config authority (update fee)
        Some(&context.accounts.payer.key()), // withdraw authority (withdraw fees)
        transfer_fee_basis_points,       // transfer fee basis points (% fee per transfer)
        maximum_fee,                     // maximum fee (maximum units of token per transfer)
    )?;

    // Initialize the standard mint account data
    initialize_mint2(
        CpiContext::new(
            context.accounts.token_program.key(),
            InitializeMint2 {
                mint: context.accounts.mint_account.to_account_info(),
            },
        ),
        2,                               // decimals
        &context.accounts.payer.key(),       // mint authority
        Some(&context.accounts.payer.key()), // freeze authority
    )?;

    handle_check_mint_data(&context.accounts)?;
    Ok(())
}

// helper to demonstrate how to read mint extension data within a program
pub fn handle_check_mint_data(accounts: &Initialize) -> Result<()> {
    let mint = &accounts.mint_account.to_account_info();
    let mint_data = mint.data.borrow();
    let mint_with_extension = StateWithExtensions::<MintState>::unpack(&mint_data)?;
    let extension_data = mint_with_extension.get_extension::<TransferFeeConfig>()?;

    assert_eq!(
        extension_data.transfer_fee_config_authority,
        OptionalNonZeroPubkey::try_from(Some(accounts.payer.key()))?
    );

    assert_eq!(
        extension_data.withdraw_withheld_authority,
        OptionalNonZeroPubkey::try_from(Some(accounts.payer.key()))?
    );

    msg!("{:?}", extension_data);
    Ok(())
}

