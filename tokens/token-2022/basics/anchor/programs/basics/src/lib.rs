use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    self, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
};

declare_id!("6qNqxkRF791FXFeQwqYQLEzAbGiqDULC5SSHVsfRoG89");

#[program]
pub mod anchor {

    use super::*;

    pub fn create_token(_context: Context<CreateTokenAccountConstraints>, _token_name: String) -> Result<()> {
        msg!("Create Token");
        Ok(())
    }
    pub fn create_token_account(_context: Context<CreateTokenAccountAccountConstraints>) -> Result<()> {
        msg!("Create Token Account");
        Ok(())
    }
    pub fn create_associated_token_account(
        _context: Context<CreateAssociatedTokenAccountAccountConstraints>,
    ) -> Result<()> {
        msg!("Create Associated Token Account");
        Ok(())
    }
    pub fn transfer_token(context: Context<TransferTokenAccountConstraints>, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: context.accounts.from.to_account_info().clone(),
            mint: context.accounts.mint.to_account_info().clone(),
            to: context.accounts.to_ata.to_account_info().clone(),
            authority: context.accounts.signer.to_account_info(),
        };
        let cpi_program = context.accounts.token_program.key();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        token_interface::transfer_checked(cpi_context, amount, context.accounts.mint.decimals)?;
        msg!("Transfer Token");
        Ok(())
    }
    pub fn mint_token(context: Context<MintTokenAccountConstraints>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: context.accounts.mint.to_account_info().clone(),
            to: context.accounts.receiver.to_account_info().clone(),
            authority: context.accounts.signer.to_account_info(),
        };
        let cpi_program = context.accounts.token_program.key();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        token_interface::mint_to(cpi_context, amount)?;
        msg!("Mint Token");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(token_name: String)]
pub struct CreateTokenAccountConstraints<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = signer.key(),
        seeds = [b"token-2022-token", signer.key().as_ref(), token_name.as_bytes()],
        bump,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct CreateTokenAccountAccountConstraints<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        token::mint = mint,
        token::authority = signer,
        payer = signer,
        seeds = [b"token-2022-token-account", signer.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct CreateAssociatedTokenAccountAccountConstraints<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        associated_token::mint = mint,
        payer = signer,
        associated_token::authority = signer,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]

pub struct TransferTokenAccountConstraints<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub from: InterfaceAccount<'info, TokenAccount>,
    pub to: SystemAccount<'info>,
    #[account(
        init,
        associated_token::mint = mint,
        payer = signer,
        associated_token::authority = to
    )]
    pub to_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct MintTokenAccountConstraints<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub receiver: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}
