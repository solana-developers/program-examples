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
        handle_initialize(&mut ctx.accounts, &name[..nl], &symbol[..sl], &uri[..ul])
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

#[inline(always)]
pub fn handle_initialize(
    accounts: &Initialize,
    name: &[u8],
    symbol: &[u8],
    uri: &[u8],
) -> Result<(), ProgramError> {
    // 165 (base) + 1 (AccountType) + 68 (MetadataPointer TLV: 2+2+64) = 234 bytes
    let mint_size_base: usize = 234;

    // TokenMetadata TLV: 2 (type) + 2 (length) + data
    // data: update_auth(32) + mint(32) + name(4+len) + symbol(4+len) + uri(4+len) + additional(4)
    let metadata_data_len = 32 + 32 + 4 + name.len() + 4 + symbol.len() + 4 + uri.len() + 4;
    let mint_size_full = mint_size_base + 4 + metadata_data_len;
    let lamports = Rent::get()?.try_minimum_balance(mint_size_full)?;

    // Create at base size; TokenMetadataInitialize will realloc to mint_size_full.
    // Lamports are pre-funded for the full size so the realloc has sufficient rent.
    accounts
        .system_program
        .create_account(
            accounts.payer,
            accounts.mint_account,
            lamports,
            mint_size_base as u64,
            accounts.token_program.to_account_view().address(),
        )
        .invoke()?;

    // InitializeMetadataPointer: opcode 39, sub-opcode 0.
    // Uses OptionalNonZeroPubkey (32 bytes each, all-zeros = None).
    // Layout: [39, 0, authority(32), metadata_address(32)] = 66 bytes
    let mut mp_data = [0u8; 66];
    mp_data[0] = 39;
    mp_data[1] = 0;
    mp_data[2..34].copy_from_slice(accounts.payer.to_account_view().address().as_ref());
    mp_data[34..66]
        .copy_from_slice(accounts.mint_account.to_account_view().address().as_ref());

    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [InstructionAccount::writable(
            accounts.mint_account.to_account_view().address(),
        )],
        [accounts.mint_account.to_account_view()],
        mp_data,
    )
    .invoke()?;

    // InitializeMint2: opcode 20
    let mut mint_data = [0u8; 67];
    mint_data[0] = 20;
    mint_data[1] = 2; // decimals
    mint_data[2..34].copy_from_slice(accounts.payer.to_account_view().address().as_ref());
    mint_data[34] = 0; // no freeze authority

    CpiCall::new(
        accounts.token_program.to_account_view().address(),
        [InstructionAccount::writable(
            accounts.mint_account.to_account_view().address(),
        )],
        [accounts.mint_account.to_account_view()],
        mint_data,
    )
    .invoke()?;

    // TokenMetadataInitialize via spl-token-metadata-interface discriminator format.
    // Token-2022 v7 uses 8-byte SHA256 discriminators for TokenMetadata/TokenGroup
    // instructions rather than simple opcode bytes.
    // Discriminator = sha256("spl_token_metadata_interface:initialize_account")[0..8]
    //               = [210, 225, 30, 162, 88, 184, 77, 141]
    // Data: [discriminator(8), name_len(u32 LE), name, symbol_len(u32 LE), symbol,
    //        uri_len(u32 LE), uri]
    // (update_authority and mint are passed as accounts, not instruction data)
    // Accounts: [metadata(=mint, writable), update_authority(readonly),
    //            mint(writable, same as metadata), mint_authority(signer)]
    // mint must be writable (not readonly) to avoid InvalidRealloc: the Solana
    // runtime rejects resizing an account that's also aliased as readonly.
    const MAX_META_IX: usize = 512;
    let mut buf = [0u8; MAX_META_IX];
    let mut pos = 0usize;
    let discriminator: [u8; 8] = [210, 225, 30, 162, 88, 184, 77, 141];
    buf[pos..pos + 8].copy_from_slice(&discriminator);
    pos += 8;
    buf[pos..pos + 4].copy_from_slice(&(name.len() as u32).to_le_bytes());
    pos += 4;
    buf[pos..pos + name.len()].copy_from_slice(name);
    pos += name.len();
    buf[pos..pos + 4].copy_from_slice(&(symbol.len() as u32).to_le_bytes());
    pos += 4;
    buf[pos..pos + symbol.len()].copy_from_slice(symbol);
    pos += symbol.len();
    buf[pos..pos + 4].copy_from_slice(&(uri.len() as u32).to_le_bytes());
    pos += 4;
    buf[pos..pos + uri.len()].copy_from_slice(uri);
    pos += uri.len();

    quasar_lang::cpi::BufCpiCall::new(
        accounts.token_program.to_account_view().address(),
        [
            InstructionAccount::writable(accounts.mint_account.to_account_view().address()),
            InstructionAccount::readonly(accounts.payer.to_account_view().address()),
            InstructionAccount::writable(accounts.mint_account.to_account_view().address()),
            InstructionAccount::readonly_signer(accounts.payer.to_account_view().address()),
        ],
        [
            accounts.mint_account.to_account_view(),
            accounts.payer.to_account_view(),
            accounts.mint_account.to_account_view(),
            accounts.payer.to_account_view(),
        ],
        buf,
        pos,
    )
    .invoke()
}
