#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;
use quasar_spl::{Mint, Token, TokenCpi};

#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

/// Demonstrates creating a mint, minting tokens, and transferring between accounts.
///
/// The Anchor version uses Metaplex for on-chain metadata. Quasar does not have
/// a Metaplex integration crate, so this example focuses on the core SPL Token
/// operations: minting and transferring.
#[program]
mod quasar_transfer_tokens {
    use super::*;

    /// Mint tokens to a recipient's token account.
    #[instruction(discriminator = 0)]
    pub fn mint_tokens(ctx: Ctx<MintTokens>, amount: u64) -> Result<(), ProgramError> {
        handle_mint_tokens(&mut ctx.accounts, amount)
    }

    /// Transfer tokens from sender to recipient.
    #[instruction(discriminator = 1)]
    pub fn transfer_tokens(ctx: Ctx<TransferTokens>, amount: u64) -> Result<(), ProgramError> {
        handle_transfer_tokens(&mut ctx.accounts, amount)
    }
}

/// Accounts for minting tokens to a recipient.
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint_authority: &'info Signer,
    #[account(mut)]
    pub mint: &'info mut Account<Mint>,
    /// The recipient's token account. Must already exist.
    #[account(mut)]
    pub recipient_token_account: &'info mut Account<Token>,
    pub token_program: &'info Program<Token>,
}

#[inline(always)]
pub fn handle_mint_tokens(accounts: &mut MintTokens, amount: u64) -> Result<(), ProgramError> {
    accounts.token_program
        .mint_to(accounts.mint, accounts.recipient_token_account, accounts.mint_authority, amount)
        .invoke()
}

/// Accounts for transferring tokens between two token accounts.
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub sender: &'info Signer,
    #[account(mut)]
    pub sender_token_account: &'info mut Account<Token>,
    #[account(mut)]
    pub recipient_token_account: &'info mut Account<Token>,
    pub token_program: &'info Program<Token>,
}

#[inline(always)]
pub fn handle_transfer_tokens(accounts: &mut TransferTokens, amount: u64) -> Result<(), ProgramError> {
    accounts.token_program
        .transfer(accounts.sender_token_account, accounts.recipient_token_account, accounts.sender, amount)
        .invoke()
}
