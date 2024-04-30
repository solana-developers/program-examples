use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::{
        extension::{
            transfer_fee::TransferFeeConfig, BaseStateWithExtensions, StateWithExtensions,
        },
        state::Mint as MintState,
    },
    token_interface::{
        transfer_checked_with_fee, Mint, Token2022, TokenAccount, TransferCheckedWithFee,
    },
};

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    pub recipient: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = sender,
        associated_token::token_program = token_program
    )]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
        associated_token::token_program = token_program
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

// transfer fees are automatically deducted from the transfer amount
// recipients receives (transfer amount - fees)
// transfer fees are stored directly on the recipient token account and must be "harvested"
pub fn process_transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    // read mint account extension data
    let mint = &ctx.accounts.mint_account.to_account_info();
    let mint_data = mint.data.borrow();
    let mint_with_extension = StateWithExtensions::<MintState>::unpack(&mint_data)?;
    let extension_data = mint_with_extension.get_extension::<TransferFeeConfig>()?;

    // calculate expected fee
    let epoch = Clock::get()?.epoch;
    let fee = extension_data.calculate_epoch_fee(epoch, amount).unwrap();

    // mint account decimals
    let decimals = ctx.accounts.mint_account.decimals;

    transfer_checked_with_fee(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferCheckedWithFee {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                source: ctx.accounts.sender_token_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                destination: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        amount,   // transfer amount
        decimals, // decimals
        fee,      // fee
    )?;

    msg!("transfer amount {}", amount);
    msg!("fee amount {}", fee);

    Ok(())
}
