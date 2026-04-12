#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;
use quasar_spl::{Mint, Token, TokenCpi};

#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

/// Demonstrates using a PDA as the mint authority for an SPL token.
///
/// The mint account itself is at a PDA address derived from `["mint"]`.
/// The same PDA serves as both the mint address AND the mint authority,
/// so minting requires PDA signing.
///
/// The Anchor version uses Metaplex for on-chain metadata. Quasar does not have
/// a Metaplex integration crate, so this example focuses on the PDA-as-authority
/// pattern.
#[program]
mod quasar_pda_mint_authority {
    use super::*;

    /// Create a token mint at a PDA. The PDA is its own mint authority.
    #[instruction(discriminator = 0)]
    pub fn create_mint(ctx: Ctx<CreateMint>, _decimals: u8) -> Result<(), ProgramError> {
        ctx.accounts.create_mint()
    }

    /// Mint tokens using the PDA mint authority.
    #[instruction(discriminator = 1)]
    pub fn mint_tokens(ctx: Ctx<MintTokens>, amount: u64) -> Result<(), ProgramError> {
        ctx.accounts.mint_tokens(amount, ctx.bumps.mint)
    }
}

/// Create the mint at a PDA. The mint authority is the mint PDA itself.
#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    /// The mint account at PDA ["mint"]. Its authority is set to itself.
    #[account(mut, init, payer = payer, seeds = [b"mint"], bump, mint::decimals = 9, mint::authority = mint)]
    pub mint: &'info mut Account<Mint>,
    pub rent: &'info Sysvar<Rent>,
    pub token_program: &'info Program<Token>,
    pub system_program: &'info Program<System>,
}

impl CreateMint<'_> {
    #[inline(always)]
    pub fn create_mint(&self) -> Result<(), ProgramError> {
        // Mint is created and initialised by Quasar's #[account(init)].
        Ok(())
    }
}

/// Mint tokens to a token account, signing with the PDA mint authority.
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    /// The PDA mint whose authority is itself.
    #[account(mut, seeds = [b"mint"], bump)]
    pub mint: &'info mut Account<Mint>,
    /// Recipient token account (must already exist).
    #[account(mut)]
    pub token_account: &'info mut Account<Token>,
    pub token_program: &'info Program<Token>,
}

impl MintTokens<'_> {
    #[inline(always)]
    pub fn mint_tokens(&mut self, amount: u64, mint_bump: u8) -> Result<(), ProgramError> {
        // The PDA mint is its own authority. Build signer seeds.
        let bump = [mint_bump];
        let seeds: &[Seed] = &[
            Seed::from(b"mint" as &[u8]),
            Seed::from(&bump as &[u8]),
        ];

        self.token_program
            .mint_to(self.mint, self.token_account, self.mint, amount)
            .invoke_signed(seeds)
    }
}
