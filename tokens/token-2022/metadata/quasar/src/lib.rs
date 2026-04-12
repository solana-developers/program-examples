#![cfg_attr(not(test), no_std)]

use quasar_lang::{
    cpi::{CpiCall, InstructionAccount},
    prelude::*,
    sysvars::Sysvar,
};

#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

pub struct Token2022Program;
impl Id for Token2022Program {
    const ID: Address = Address::new_from_array([
        6, 221, 246, 225, 238, 117, 143, 222, 24, 66, 93, 188, 228, 108, 205, 218,
        182, 26, 252, 77, 131, 185, 13, 39, 254, 189, 249, 40, 216, 161, 139, 252,
    ]);
}

/// Maximum length for name, symbol, and URI fields.
const MAX_NAME: usize = 32;
const MAX_SYMBOL: usize = 10;
const MAX_URI: usize = 128;

/// Demonstrates the Token-2022 MetadataPointer + TokenMetadata extensions.
/// Creates a mint with embedded on-chain metadata (name, symbol, URI).
///
/// Uses fixed-size byte arrays for the metadata fields since Quasar
/// deserializes all instruction arguments at entry.
#[program]
mod quasar_metadata {
    use super::*;

    /// Create a mint with MetadataPointer extension, then initialize
    /// token metadata via Token-2022's native metadata instruction.
    ///
    /// * `name` — token name, right-padded with zeroes
    /// * `name_len` — actual byte length of the name
    /// * `symbol` — ticker, right-padded with zeroes
    /// * `symbol_len` — actual byte length of the symbol
    /// * `uri` — metadata URI, right-padded with zeroes
    /// * `uri_len` — actual byte length of the URI
    #[instruction(discriminator = 0)]
    pub fn initialize(
        ctx: Ctx<Initialize>,
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
        ctx.accounts
            .initialize(&name[..nl], &symbol[..sl], &uri[..ul])
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    #[account(mut)]
    pub mint_account: &'info Signer,
    pub token_program: &'info Program<Token2022Program>,
    pub system_program: &'info Program<System>,
}

impl Initialize<'_> {
    #[inline(always)]
    pub fn initialize(
        &self,
        name: &[u8],
        symbol: &[u8],
        uri: &[u8],
    ) -> Result<(), ProgramError> {
        // Calculate the total metadata size.
        // MetadataPointer (64 bytes) + TLV overhead + actual metadata
        // Metadata format: 4 (TLV header) + 32 (update_auth) + 32 (mint)
        //   + 4 + name.len + 4 + symbol.len + 4 + uri.len + 4 + 0 (additional metadata)
        let metadata_data_len = 32 + 32 + 4 + name.len() + 4 + symbol.len() + 4 + uri.len() + 4;
        let total_ext_data = 4 + metadata_data_len; // TLV: 2 type + 2 length + data
        // Mint base (82) + padding (82) + AccountType (1) + MetadataPointer ext (68) + metadata TLV
        let mint_size = 82 + 82 + 1 + 68 + total_ext_data;
        let lamports = Rent::get()?.try_minimum_balance(mint_size)?;

        self.system_program
            .create_account(
                self.payer,
                self.mint_account,
                lamports,
                mint_size as u64,
                self.token_program.to_account_view().address(),
            )
            .invoke()?;

        // InitializeMetadataPointer: opcode 39, sub-opcode 0.
        let mut mp_data = [0u8; 66];
        mp_data[0] = 39;
        mp_data[1] = 0;
        mp_data[2..34].copy_from_slice(self.payer.to_account_view().address().as_ref());
        mp_data[34..66]
            .copy_from_slice(self.mint_account.to_account_view().address().as_ref());

        CpiCall::new(
            self.token_program.to_account_view().address(),
            [InstructionAccount::writable(
                self.mint_account.to_account_view().address(),
            )],
            [self.mint_account.to_account_view()],
            mp_data,
        )
        .invoke()?;

        // InitializeMint2.
        let mut mint_data = [0u8; 67];
        mint_data[0] = 20; // InitializeMint2
        mint_data[1] = 2; // decimals
        mint_data[2..34].copy_from_slice(self.payer.to_account_view().address().as_ref());
        mint_data[34] = 0; // no freeze authority

        CpiCall::new(
            self.token_program.to_account_view().address(),
            [InstructionAccount::writable(
                self.mint_account.to_account_view().address(),
            )],
            [self.mint_account.to_account_view()],
            mint_data,
        )
        .invoke()?;

        // TokenMetadataInitialize: TokenInstruction::TokenMetadataExtension = 44
        // Sub-instruction: Initialize = 0
        // Layout: [44, 0, update_authority(32), mint(32),
        //          name_len(u32 LE), name, symbol_len(u32 LE), symbol,
        //          uri_len(u32 LE), uri]
        const MAX_META_IX: usize = 512;
        let mut buf = [0u8; MAX_META_IX];
        let mut pos = 0usize;
        buf[pos] = 44;
        pos += 1;
        buf[pos] = 0;
        pos += 1;
        // update_authority
        buf[pos..pos + 32].copy_from_slice(self.payer.to_account_view().address().as_ref());
        pos += 32;
        // mint
        buf[pos..pos + 32]
            .copy_from_slice(self.mint_account.to_account_view().address().as_ref());
        pos += 32;
        // name
        buf[pos..pos + 4].copy_from_slice(&(name.len() as u32).to_le_bytes());
        pos += 4;
        buf[pos..pos + name.len()].copy_from_slice(name);
        pos += name.len();
        // symbol
        buf[pos..pos + 4].copy_from_slice(&(symbol.len() as u32).to_le_bytes());
        pos += 4;
        buf[pos..pos + symbol.len()].copy_from_slice(symbol);
        pos += symbol.len();
        // uri
        buf[pos..pos + 4].copy_from_slice(&(uri.len() as u32).to_le_bytes());
        pos += 4;
        buf[pos..pos + uri.len()].copy_from_slice(uri);
        pos += uri.len();

        quasar_lang::cpi::BufCpiCall::new(
            self.token_program.to_account_view().address(),
            [
                InstructionAccount::writable(
                    self.mint_account.to_account_view().address(),
                ),
                InstructionAccount::readonly_signer(
                    self.payer.to_account_view().address(),
                ),
                InstructionAccount::readonly_signer(
                    self.payer.to_account_view().address(),
                ),
            ],
            [
                self.mint_account.to_account_view(),
                self.payer.to_account_view(),
                self.payer.to_account_view(),
            ],
            buf,
            pos,
        )
        .invoke()
    }
}
