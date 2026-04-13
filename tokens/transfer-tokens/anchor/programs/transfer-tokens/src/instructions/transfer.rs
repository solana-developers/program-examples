use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
pub struct TransferTokensAccountConstraints<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    pub recipient: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = sender,
    )]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handle_transfer_tokens(context: Context<TransferTokensAccountConstraints>, amount: u64) -> Result<()> {
    msg!("Transferring tokens...");
    msg!(
        "Mint: {}",
        &context.accounts.mint_account.to_account_info().key()
    );
    msg!(
        "From Token Address: {}",
        &context.accounts.sender_token_account.key()
    );
    msg!(
        "To Token Address: {}",
        &context.accounts.recipient_token_account.key()
    );

    // Invoke the transfer instruction on the token program
    transfer(
        CpiContext::new(
            context.accounts.token_program.key(),
            Transfer {
                from: context.accounts.sender_token_account.to_account_info(),
                to: context.accounts.recipient_token_account.to_account_info(),
                authority: context.accounts.sender.to_account_info(),
            },
        ),
        amount * 10u64.pow(context.accounts.mint_account.decimals as u32), // Transfer amount, adjust for decimals
    )?;

    msg!("Tokens transferred successfully.");

    Ok(())
}
