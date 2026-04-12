use crate::bubblegum_types::encode_mint_to_collection_v1;
use crate::*;
use quasar_lang::cpi::{InstructionAccount, InstructionView};

/// Maximum CPI accounts for MintToCollectionV1: 16 fixed accounts.
const MINT_CPI_ACCOUNTS: usize = 16;

/// Maximum URI length for the instruction data buffer.
const MAX_URI_LEN: usize = 256;

/// Maximum instruction data buffer: discriminator(8) + metadata overhead(~120) + URI.
const MAX_IX_DATA: usize = 400;

/// Accounts for minting a compressed NFT to a collection.
#[derive(Accounts)]
pub struct Mint<'info> {
    pub payer: &'info Signer,
    /// Tree authority PDA (seeds checked by Bubblegum).
    #[account(mut)]
    pub tree_authority: &'info UncheckedAccount,
    /// Owner of the newly minted cNFT.
    pub leaf_owner: &'info UncheckedAccount,
    /// Delegate for the newly minted cNFT.
    pub leaf_delegate: &'info UncheckedAccount,
    /// Merkle tree to mint into.
    #[account(mut)]
    pub merkle_tree: &'info UncheckedAccount,
    /// Tree delegate (must be signer).
    pub tree_delegate: &'info Signer,
    /// Collection authority (must be signer).
    pub collection_authority: &'info Signer,
    /// Collection authority record PDA (or Bubblegum program address).
    pub collection_authority_record_pda: &'info UncheckedAccount,
    /// Collection mint account.
    pub collection_mint: &'info UncheckedAccount,
    /// Collection metadata account.
    #[account(mut)]
    pub collection_metadata: &'info UncheckedAccount,
    /// Edition account for the collection.
    pub edition_account: &'info UncheckedAccount,
    /// Bubblegum signer PDA.
    pub bubblegum_signer: &'info UncheckedAccount,
    /// SPL Noop log wrapper.
    pub log_wrapper: &'info UncheckedAccount,
    /// SPL Account Compression program.
    #[account(address = SPL_ACCOUNT_COMPRESSION_ID)]
    pub compression_program: &'info UncheckedAccount,
    /// Token Metadata program.
    pub token_metadata_program: &'info UncheckedAccount,
    /// mpl-bubblegum program.
    #[account(address = MPL_BUBBLEGUM_ID)]
    pub bubblegum_program: &'info UncheckedAccount,
    pub system_program: &'info Program<System>,
}

impl<'info> Mint<'info> {
    pub fn mint(&self, ctx: &Ctx<'info, Mint<'info>>) -> Result<(), ProgramError> {
        // Parse URI from instruction data: u32 length prefix + utf8 bytes (borsh String)
        let data = ctx.data;
        if data.len() < 4 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let uri_len = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
        if data.len() < 4 + uri_len || uri_len > MAX_URI_LEN {
            return Err(ProgramError::InvalidInstructionData);
        }
        let uri = &data[4..4 + uri_len];

        // Build CPI instruction data
        let mut ix_data = [0u8; MAX_IX_DATA];
        let ix_len = encode_mint_to_collection_v1(
            &mut ix_data,
            uri,
            self.collection_authority.address(),
            self.collection_mint.address(),
        );

        // Build instruction account metas matching MintToCollectionV1 layout
        let ix_accounts: [InstructionAccount; MINT_CPI_ACCOUNTS] = [
            InstructionAccount::writable(self.tree_authority.address()),
            InstructionAccount::readonly(self.leaf_owner.address()),
            InstructionAccount::readonly(self.leaf_delegate.address()),
            InstructionAccount::writable(self.merkle_tree.address()),
            InstructionAccount::readonly_signer(self.payer.address()),
            InstructionAccount::readonly_signer(self.tree_delegate.address()),
            InstructionAccount::readonly_signer(self.collection_authority.address()),
            InstructionAccount::readonly(self.collection_authority_record_pda.address()),
            InstructionAccount::readonly(self.collection_mint.address()),
            InstructionAccount::writable(self.collection_metadata.address()),
            InstructionAccount::readonly(self.edition_account.address()),
            InstructionAccount::readonly(self.bubblegum_signer.address()),
            InstructionAccount::readonly(self.log_wrapper.address()),
            InstructionAccount::readonly(self.compression_program.address()),
            InstructionAccount::readonly(self.token_metadata_program.address()),
            InstructionAccount::readonly(self.system_program.address()),
        ];

        let views: [AccountView; MINT_CPI_ACCOUNTS] = [
            self.tree_authority.to_account_view().clone(),
            self.leaf_owner.to_account_view().clone(),
            self.leaf_delegate.to_account_view().clone(),
            self.merkle_tree.to_account_view().clone(),
            self.payer.to_account_view().clone(),
            self.tree_delegate.to_account_view().clone(),
            self.collection_authority.to_account_view().clone(),
            self.collection_authority_record_pda.to_account_view().clone(),
            self.collection_mint.to_account_view().clone(),
            self.collection_metadata.to_account_view().clone(),
            self.edition_account.to_account_view().clone(),
            self.bubblegum_signer.to_account_view().clone(),
            self.log_wrapper.to_account_view().clone(),
            self.compression_program.to_account_view().clone(),
            self.token_metadata_program.to_account_view().clone(),
            self.system_program.to_account_view().clone(),
        ];

        let instruction = InstructionView {
            program_id: &MPL_BUBBLEGUM_ID,
            data: &ix_data[..ix_len],
            accounts: &ix_accounts,
        };

        solana_instruction_view::cpi::invoke::<MINT_CPI_ACCOUNTS, AccountView>(
            &instruction,
            &views,
        )
    }
}
