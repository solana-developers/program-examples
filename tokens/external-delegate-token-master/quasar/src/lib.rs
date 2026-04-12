#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;
use quasar_spl::{Token, TokenCpi};

#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

/// User account storing the Solana authority and linked Ethereum address.
#[account(discriminator = 1)]
pub struct UserAccount {
    pub authority: Address,
    pub ethereum_address: [u8; 20],
}

/// External delegate token master: allows transfers authorised either by
/// the Solana authority or by an Ethereum signature (secp256k1).
#[program]
mod quasar_external_delegate_token_master {
    use super::*;

    /// Initialize a user account with zero Ethereum address.
    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>) -> Result<(), ProgramError> {
        ctx.accounts.initialize()
    }

    /// Set the Ethereum address for signature verification.
    #[instruction(discriminator = 1)]
    pub fn set_ethereum_address(
        ctx: Ctx<SetEthereumAddress>,
        ethereum_address: [u8; 20],
    ) -> Result<(), ProgramError> {
        ctx.accounts.set_ethereum_address(ethereum_address)
    }

    /// Transfer tokens using an Ethereum signature for authorisation.
    #[instruction(discriminator = 2)]
    pub fn transfer_tokens(
        ctx: Ctx<TransferTokens>,
        amount: u64,
        signature: [u8; 65],
        message: [u8; 32],
    ) -> Result<(), ProgramError> {
        ctx.accounts
            .transfer_tokens(amount, &signature, &message, &ctx.bumps)
    }

    /// Transfer tokens using the Solana authority directly.
    #[instruction(discriminator = 3)]
    pub fn authority_transfer(
        ctx: Ctx<AuthorityTransfer>,
        amount: u64,
    ) -> Result<(), ProgramError> {
        ctx.accounts.authority_transfer(amount, &ctx.bumps)
    }
}

// ---------------------------------------------------------------------------
// Instruction accounts
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, init, payer = authority)]
    pub user_account: &'info mut Account<UserAccount>,
    #[account(mut)]
    pub authority: &'info Signer,
    pub system_program: &'info Program<System>,
}

impl Initialize<'_> {
    #[inline(always)]
    pub fn initialize(&mut self) -> Result<(), ProgramError> {
        self.user_account
            .set_inner(*self.authority.address(), [0u8; 20]);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetEthereumAddress<'info> {
    #[account(mut)]
    pub user_account: &'info mut Account<UserAccount>,
    pub authority: &'info Signer,
}

impl SetEthereumAddress<'_> {
    #[inline(always)]
    pub fn set_ethereum_address(
        &mut self,
        ethereum_address: [u8; 20],
    ) -> Result<(), ProgramError> {
        require_keys_eq!(
            self.user_account.authority,
            *self.authority.address(),
            ProgramError::MissingRequiredSignature
        );
        self.user_account.ethereum_address = ethereum_address;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    pub user_account: &'info Account<UserAccount>,
    pub authority: &'info Signer,
    #[account(mut)]
    pub user_token_account: &'info mut Account<Token>,
    #[account(mut)]
    pub recipient_token_account: &'info mut Account<Token>,
    /// PDA derived from user_account address.
    #[account(seeds = [user_account], bump)]
    pub user_pda: &'info UncheckedAccount,
    pub token_program: &'info Program<Token>,
}

impl TransferTokens<'_> {
    #[inline(always)]
    pub fn transfer_tokens(
        &self,
        amount: u64,
        signature: &[u8; 65],
        message: &[u8; 32],
        bumps: &TransferTokensBumps,
    ) -> Result<(), ProgramError> {
        if !verify_ethereum_signature(
            &self.user_account.ethereum_address,
            message,
            signature,
        ) {
            return Err(ProgramError::Custom(1)); // InvalidSignature
        }

        let bump = [bumps.user_pda];
        let seeds: &[Seed] = &[
            Seed::from(self.user_account.address().as_ref()),
            Seed::from(&bump as &[u8]),
        ];

        self.token_program
            .transfer(
                self.user_token_account,
                self.recipient_token_account,
                self.user_pda,
                amount,
            )
            .invoke_signed(seeds)
    }
}

#[derive(Accounts)]
pub struct AuthorityTransfer<'info> {
    pub user_account: &'info Account<UserAccount>,
    pub authority: &'info Signer,
    #[account(mut)]
    pub user_token_account: &'info mut Account<Token>,
    #[account(mut)]
    pub recipient_token_account: &'info mut Account<Token>,
    /// PDA derived from user_account address.
    #[account(seeds = [user_account], bump)]
    pub user_pda: &'info UncheckedAccount,
    pub token_program: &'info Program<Token>,
}

impl AuthorityTransfer<'_> {
    #[inline(always)]
    pub fn authority_transfer(
        &self,
        amount: u64,
        bumps: &AuthorityTransferBumps,
    ) -> Result<(), ProgramError> {
        require_keys_eq!(
            self.user_account.authority,
            *self.authority.address(),
            ProgramError::MissingRequiredSignature
        );

        let bump = [bumps.user_pda];
        let seeds: &[Seed] = &[
            Seed::from(self.user_account.address().as_ref()),
            Seed::from(&bump as &[u8]),
        ];

        self.token_program
            .transfer(
                self.user_token_account,
                self.recipient_token_account,
                self.user_pda,
                amount,
            )
            .invoke_signed(seeds)
    }
}

// ---------------------------------------------------------------------------
// Ethereum signature verification using raw syscalls
// ---------------------------------------------------------------------------

fn keccak256(data: &[u8]) -> [u8; 32] {
    let hash = solana_keccak_hasher::hash(data);
    let bytes: &[u8] = hash.as_ref();
    let mut result = [0u8; 32];
    result.copy_from_slice(bytes);
    result
}

/// Recover secp256k1 public key from a signature, using the raw Solana syscall.
///
/// Returns `None` if recovery fails. The returned key is the 65-byte
/// uncompressed public key (first byte `0x04` is omitted by the syscall,
/// only the 64 bytes of x||y are returned).
fn secp256k1_recover(
    message_hash: &[u8; 32],
    recovery_id: u8,
    signature: &[u8; 64],
) -> Option<[u8; 64]> {
    #[cfg(target_os = "solana")]
    {
        let mut pubkey_result = [0u8; 64];
        let rc = unsafe {
            solana_define_syscall::definitions::sol_secp256k1_recover(
                message_hash.as_ptr(),
                recovery_id as u64,
                signature.as_ptr(),
                pubkey_result.as_mut_ptr(),
            )
        };
        if rc == 0 {
            Some(pubkey_result)
        } else {
            None
        }
    }
    #[cfg(not(target_os = "solana"))]
    {
        // Off-chain: not implemented (would need a secp256k1 library).
        let _ = (message_hash, recovery_id, signature);
        None
    }
}

fn verify_ethereum_signature(
    ethereum_address: &[u8; 20],
    message: &[u8; 32],
    signature: &[u8; 65],
) -> bool {
    let recovery_id = signature[64];
    let mut sig = [0u8; 64];
    sig.copy_from_slice(&signature[..64]);

    if let Some(pubkey_bytes) = secp256k1_recover(message, recovery_id, &sig) {
        // Ethereum address = last 20 bytes of keccak256(public_key)
        // The syscall returns the 64-byte uncompressed key (sans prefix byte).
        let hash = keccak256(&pubkey_bytes);
        let mut recovered_address = [0u8; 20];
        recovered_address.copy_from_slice(&hash[12..]);
        recovered_address == *ethereum_address
    } else {
        false
    }
}
