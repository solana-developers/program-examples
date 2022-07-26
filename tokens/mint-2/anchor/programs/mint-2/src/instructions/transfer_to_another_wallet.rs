use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token,
        associated_token,
    },
};


pub fn transfer_to_another_wallet(
    ctx: Context<TransferToAnotherWallet>, 
    amount: u64,
) -> Result<()> {

    msg!("Transferring {} tokens to new token account...", amount);
    msg!("Mint: {}", &ctx.accounts.mint_account.to_account_info().key());   
    msg!("Owner Token Address: {}", &ctx.accounts.owner_token_account.key());  
    msg!("Recipient Token Address: {}", &ctx.accounts.recipient_token_account.key());
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.owner_token_account.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    msg!("Tokens transferred to wallet successfully.");

    Ok(())
}


#[derive(Accounts)]
pub struct TransferToAnotherWallet<'info> {
    #[account(mut)]
    pub mint_account: Account<'info, token::Mint>,
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = owner,
    )]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: Crediting not Debiting
    pub recipient: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}