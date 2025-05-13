use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Transfer};
use solana_program::secp256k1_recover::secp256k1_recover;
use sha3::{Digest, Keccak256};

declare_id!("FYPkt5VWMvtyWZDMGCwoKFkE3wXTzphicTpnNGuHWVbD");

#[program]
pub mod external_delegate_token_master {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.authority = ctx.accounts.authority.key();
        user_account.ethereum_address = [0; 20];
        Ok(())
    }

    pub fn set_ethereum_address(ctx: Context<SetEthereumAddress>, ethereum_address: [u8; 20]) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.ethereum_address = ethereum_address;
        Ok(())
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64, signature: [u8; 65], message: [u8; 32]) -> Result<()> {
        let user_account = &ctx.accounts.user_account;

        if !verify_ethereum_signature(&user_account.ethereum_address, &message, &signature) {
            return Err(ErrorCode::InvalidSignature.into());
        }

        // Transfer tokens
        let transfer_instruction = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.user_pda.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_instruction,
                &[&[
                    user_account.key().as_ref(),
                    &[ctx.bumps.user_pda],
                ]],
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn authority_transfer(ctx: Context<AuthorityTransfer>, amount: u64) -> Result<()> {
        // Transfer tokens
        let transfer_instruction = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.user_pda.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_instruction,
                &[&[
                    ctx.accounts.user_account.key().as_ref(),
                    &[ctx.bumps.user_pda],
                ]],
            ),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 20)] // Ensure this is only for user_account
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub authority: Signer<'info>, // This should remain as a signer
    pub system_program: Program<'info, System>, // Required for initialization
}

#[derive(Accounts)]
pub struct SetEthereumAddress<'info> {
    #[account(mut, has_one = authority)]
    pub user_account: Account<'info, UserAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(has_one = authority)]
    pub user_account: Account<'info, UserAccount>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [user_account.key().as_ref()],
        bump,
    )]
    pub user_pda: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AuthorityTransfer<'info> {
    #[account(has_one = authority)]
    pub user_account: Account<'info, UserAccount>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [user_account.key().as_ref()],
        bump,
    )]
    pub user_pda: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct UserAccount {
    pub authority: Pubkey,
    pub ethereum_address: [u8; 20],
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Ethereum signature")]
    InvalidSignature,
}

fn verify_ethereum_signature(ethereum_address: &[u8; 20], message: &[u8; 32], signature: &[u8; 65]) -> bool {
    let recovery_id = signature[64];
    let mut sig = [0u8; 64];
    sig.copy_from_slice(&signature[..64]);

    if let Ok(pubkey) = secp256k1_recover(message, recovery_id, &sig) {
        let pubkey_bytes = pubkey.to_bytes();
        let mut recovered_address = [0u8; 20];
        recovered_address.copy_from_slice(&keccak256(&pubkey_bytes[1..])[12..]);
        recovered_address == *ethereum_address
    } else {
        false
    }
}

fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}
