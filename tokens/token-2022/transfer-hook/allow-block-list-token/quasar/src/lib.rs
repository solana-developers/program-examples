#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use constants::*;
use instructions::*;

#[cfg(test)]
mod tests;

declare_id!("3ku1ZEGvBEEfhaYsAzBZuecTPEa58ZRhoVqHVGpGxVGi");

/// Allow/Block List Token — a transfer hook program that enforces allow/block
/// lists on Token-2022 transfers using per-wallet PDA entries and mint
/// metadata to control modes (Allow, Block, Mixed/Threshold).
#[program]
mod quasar_abl_token {
    use super::*;

    /// Create a Token-2022 mint with transfer hook, permanent delegate,
    /// metadata pointer, and embedded metadata (including AB mode).
    /// Also initialises the ExtraAccountMetaList PDA.
    ///
    /// Arguments (after discriminator):
    ///   decimals: u8
    ///   freeze_authority: [u8; 32]
    ///   permanent_delegate: [u8; 32]
    ///   transfer_hook_authority: [u8; 32]
    ///   mode: u8 (0=Allow, 1=Block, 2=Mixed)
    ///   threshold: u64
    ///   name: [u8; 32], name_len: u8
    ///   symbol: [u8; 10], symbol_len: u8
    ///   uri: [u8; 128], uri_len: u8
    #[instruction(discriminator = [0, 0, 0, 0, 0, 0, 0, 0])]
    pub fn init_mint(
        ctx: Ctx<InitMint>,
        decimals: u8,
        freeze_authority: [u8; 32],
        permanent_delegate: [u8; 32],
        transfer_hook_authority: [u8; 32],
        mode: u8,
        threshold: u64,
        name: [u8; MAX_NAME],
        name_len: u8,
        symbol: [u8; MAX_SYMBOL],
        symbol_len: u8,
        uri: [u8; MAX_URI],
        uri_len: u8,
    ) -> Result<(), ProgramError> {
        let nl = name_len as usize;
        let sl = symbol_len as usize;
        let ul = uri_len as usize;
        if nl > MAX_NAME || sl > MAX_SYMBOL || ul > MAX_URI {
            return Err(ProgramError::InvalidInstructionData);
        }
        let freeze_addr = Address::new_from_array(freeze_authority);
        let delegate_addr = Address::new_from_array(permanent_delegate);
        let hook_auth_addr = Address::new_from_array(transfer_hook_authority);
        instructions::handle_init_mint(&mut ctx.accounts, decimals,
            &freeze_addr,
            &delegate_addr,
            &hook_auth_addr,
            mode,
            threshold,
            &name[..nl],
            &symbol[..sl],
            &uri[..ul],)
    }

    /// Create the Config PDA with the payer as authority.
    #[instruction(discriminator = [0, 0, 0, 0, 0, 0, 0, 1])]
    pub fn init_config(ctx: Ctx<InitConfig>) -> Result<(), ProgramError> {
        instructions::handle_init_config(&mut ctx.accounts)
    }

    /// Attach the transfer hook to an existing mint (sets the hook program_id
    /// and creates the ExtraAccountMetaList PDA).
    #[instruction(discriminator = [0, 0, 0, 0, 0, 0, 0, 2])]
    pub fn attach_to_mint(ctx: Ctx<AttachToMint>) -> Result<(), ProgramError> {
        instructions::handle_attach_to_mint(&mut ctx.accounts)
    }

    /// SPL Transfer Hook execute handler. Called by Token-2022 during
    /// transfers to enforce allow/block/threshold rules.
    /// Discriminator = sha256("spl-transfer-hook-interface:execute")[:8]
    #[instruction(discriminator = [105, 37, 101, 197, 75, 251, 102, 26])]
    pub fn tx_hook(ctx: Ctx<TxHook>, amount: u64) -> Result<(), ProgramError> {
        instructions::handle_tx_hook(&mut ctx.accounts, amount)
    }

    /// Create a per-wallet allow/block entry.
    #[instruction(discriminator = [0, 0, 0, 0, 0, 0, 0, 4])]
    pub fn init_wallet(ctx: Ctx<InitWallet>, allowed: bool) -> Result<(), ProgramError> {
        instructions::handle_init_wallet(&mut ctx.accounts, allowed)
    }

    /// Remove a wallet entry, closing the PDA account.
    #[instruction(discriminator = [0, 0, 0, 0, 0, 0, 0, 5])]
    pub fn remove_wallet(ctx: Ctx<RemoveWallet>) -> Result<(), ProgramError> {
        instructions::handle_remove_wallet(&mut ctx.accounts)
    }

    /// Change the allow/block mode on the mint's metadata.
    #[instruction(discriminator = [0, 0, 0, 0, 0, 0, 0, 6])]
    pub fn change_mode(ctx: Ctx<ChangeMode>, mode: u8, threshold: u64) -> Result<(), ProgramError> {
        instructions::handle_change_mode(&mut ctx.accounts, mode, threshold)
    }
}
