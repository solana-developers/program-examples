use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use std::vec::Vec; 

declare_id!("7tUBaLEw5BVFmqWF8ixfxVSb7WM7CqRTXMmY8kK6uf1N");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let wallet_state = &mut ctx.accounts.wallet_state;
        wallet_state.owner = *ctx.accounts.owner.key;

        let example_token_address = ctx.accounts.token_mint.key();
        wallet_state.transfer_status.push((example_token_address, true)); 
        Ok(())
    }

    pub fn toggle_transfer_switch(ctx: Context<ToggleTransferSwitch>, token_mint: Pubkey) -> Result<()> {
        let wallet_state = &mut ctx.accounts.wallet_state;

        for status in &mut wallet_state.transfer_status {
            if status.0 == token_mint {
                status.1 = !status.1;
                return Ok(());
            }
        }

        wallet_state.transfer_status.push((token_mint, true)); 
        Ok(())
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let sender_state = &ctx.accounts.sender_state;

        /// CHECK: Check if transfer is enabled for the specified token
        let token_mint: Pubkey = ctx.accounts.from.mint;
        for status in &sender_state.transfer_status {
            if status.0 == token_mint {
                if !status.1 {
                    return Err(ErrorCode::TransfersDisabled.into());
                }
                break;
            }
        }

        let cpi_accounts = token::Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + (50 * 2 * 32))] 
    pub wallet_state: Account<'info, WalletState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK:
    pub token_mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ToggleTransferSwitch<'info> {
    #[account(mut, has_one = owner)]
    pub wallet_state: Account<'info, WalletState>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(mut, has_one = owner)]
    pub sender_state: Account<'info, WalletState>,
    pub owner: Signer<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(Debug, Default)]
pub struct WalletState {
    pub owner: Pubkey,
    pub transfer_status: Vec<(Pubkey, bool)>, 
}

#[error_code]
pub enum ErrorCode {
    #[msg("Transfers are disabled for this wallet.")]
    TransfersDisabled,
    #[msg("Token not initialized for transfer management.")]
    TokenNotInitialized,
}

