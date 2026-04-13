/// Re-implementation of mpl-bubblegum types using borsh 1.x and Anchor 1.0's Pubkey.
///
/// mpl-bubblegum 2.1.1 depends on solana-program 2.x which is incompatible with
/// Anchor 1.0's solana 3.x types. These types are borsh-compatible reproductions
/// that produce identical binary serialization.
use anchor_lang::prelude::*;
use borsh::BorshSerialize;

/// Mirrors mpl_bubblegum::types::Creator
#[derive(BorshSerialize, Clone, Debug)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

/// Mirrors mpl_bubblegum::types::Collection
#[derive(BorshSerialize, Clone, Debug)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

/// Mirrors mpl_bubblegum::types::TokenProgramVersion
#[derive(BorshSerialize, Clone, Debug)]
pub enum TokenProgramVersion {
    Original,
    Token2022,
}

/// Mirrors mpl_bubblegum::types::TokenStandard
#[derive(BorshSerialize, Clone, Debug)]
pub enum TokenStandard {
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
}

/// Mirrors mpl_bubblegum::types::UseMethod
#[derive(BorshSerialize, Clone, Debug)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

/// Mirrors mpl_bubblegum::types::Uses
#[derive(BorshSerialize, Clone, Debug)]
pub struct Uses {
    pub use_method: UseMethod,
    pub remaining: u64,
    pub total: u64,
}

/// Mirrors mpl_bubblegum::types::MetadataArgs
#[derive(BorshSerialize, Clone, Debug)]
pub struct MetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<TokenStandard>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
    pub token_program_version: TokenProgramVersion,
    pub creators: Vec<Creator>,
}

/// MintToCollectionV1 instruction discriminator from mpl-bubblegum
pub const MINT_TO_COLLECTION_V1_DISCRIMINATOR: [u8; 8] = [153, 18, 178, 47, 197, 158, 86, 15];

/// MintToCollectionV1 instruction args (wraps MetadataArgs)
#[derive(BorshSerialize)]
pub struct MintToCollectionV1InstructionArgs {
    pub metadata: MetadataArgs,
}

/// Compute the leaf hash for a V1 LeafSchema, matching mpl_bubblegum::types::LeafSchema::hash().
/// Uses keccak256 over the version byte and all fields.
pub fn leaf_schema_v1_hash(
    id: &Pubkey,
    owner: &Pubkey,
    delegate: &Pubkey,
    nonce: u64,
    data_hash: &[u8; 32],
    creator_hash: &[u8; 32],
) -> [u8; 32] {
    use sha3::{Digest, Keccak256};
    let mut hasher = Keccak256::new();
    hasher.update([1u8]); // Version::V1 = 1
    hasher.update(id.as_ref());
    hasher.update(owner.as_ref());
    hasher.update(delegate.as_ref());
    hasher.update(nonce.to_le_bytes());
    hasher.update(data_hash);
    hasher.update(creator_hash);
    hasher.finalize().into()
}

/// Compute the asset id from tree and nonce, matching mpl_bubblegum::utils::get_asset_id().
pub fn get_asset_id(tree: &Pubkey, nonce: u64) -> Pubkey {
    // mpl-bubblegum program ID
    let bubblegum_id = Pubkey::new_from_array([
        0x98, 0x8b, 0x80, 0xeb, 0x79, 0x35, 0x28, 0x69, 0xb2, 0x24, 0x74, 0x5f, 0x59, 0xdd,
        0xbf, 0x8a, 0x26, 0x58, 0xca, 0x13, 0xdc, 0x68, 0x81, 0x21, 0x26, 0x35, 0x1c, 0xae,
        0x07, 0xc1, 0xa5, 0xa5,
    ]);
    Pubkey::find_program_address(&[b"asset", tree.as_ref(), &nonce.to_le_bytes()], &bubblegum_id).0
}
