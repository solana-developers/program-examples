/// Re-implementation of mpl-bubblegum types for manual serialization.
///
/// mpl-bubblegum depends on solana-program 2.x which is incompatible with
/// Quasar's solana 3.x types. These types produce borsh-compatible binary
/// serialization via manual encode methods.
use quasar_lang::prelude::*;

/// MintToCollectionV1 instruction discriminator from mpl-bubblegum.
pub const MINT_TO_COLLECTION_V1_DISCRIMINATOR: [u8; 8] = [153, 18, 178, 47, 197, 158, 86, 15];

/// Manually encode MintToCollectionV1 instruction data into a buffer.
/// Returns the number of bytes written.
///
/// Encodes MetadataArgs in borsh order with hardcoded name="BURGER",
/// symbol="BURG", seller_fee=0, NonFungible token standard, Original
/// token program version, and a single creator.
#[allow(clippy::too_many_arguments)]
pub fn encode_mint_to_collection_v1(
    buf: &mut [u8],
    uri: &[u8],
    creator_address: &Address,
    collection_mint: &Address,
) -> usize {
    let mut offset = 0;

    // Discriminator
    buf[offset..offset + 8].copy_from_slice(&MINT_TO_COLLECTION_V1_DISCRIMINATOR);
    offset += 8;

    // name: "BURGER"
    let name = b"BURGER";
    buf[offset..offset + 4].copy_from_slice(&(name.len() as u32).to_le_bytes());
    offset += 4;
    buf[offset..offset + name.len()].copy_from_slice(name);
    offset += name.len();

    // symbol: "BURG"
    let symbol = b"BURG";
    buf[offset..offset + 4].copy_from_slice(&(symbol.len() as u32).to_le_bytes());
    offset += 4;
    buf[offset..offset + symbol.len()].copy_from_slice(symbol);
    offset += symbol.len();

    // uri
    buf[offset..offset + 4].copy_from_slice(&(uri.len() as u32).to_le_bytes());
    offset += 4;
    buf[offset..offset + uri.len()].copy_from_slice(uri);
    offset += uri.len();

    // seller_fee_basis_points: u16 = 0
    buf[offset..offset + 2].copy_from_slice(&0u16.to_le_bytes());
    offset += 2;

    // primary_sale_happened: bool = false
    buf[offset] = 0;
    offset += 1;

    // is_mutable: bool = false
    buf[offset] = 0;
    offset += 1;

    // edition_nonce: Option<u8> = Some(0)
    buf[offset] = 1; // Some
    offset += 1;
    buf[offset] = 0; // value
    offset += 1;

    // token_standard: Option<TokenStandard> = Some(NonFungible = 0)
    buf[offset] = 1; // Some
    offset += 1;
    buf[offset] = 0; // NonFungible
    offset += 1;

    // collection: Option<Collection> = Some({ verified: false, key: collection_mint })
    buf[offset] = 1; // Some
    offset += 1;
    buf[offset] = 0; // verified = false
    offset += 1;
    buf[offset..offset + 32].copy_from_slice(collection_mint.as_ref());
    offset += 32;

    // uses: Option<Uses> = None
    buf[offset] = 0;
    offset += 1;

    // token_program_version: Original = 0
    buf[offset] = 0;
    offset += 1;

    // creators: Vec<Creator> len=1
    buf[offset..offset + 4].copy_from_slice(&1u32.to_le_bytes());
    offset += 4;
    // Creator { address, verified: false, share: 100 }
    buf[offset..offset + 32].copy_from_slice(creator_address.as_ref());
    offset += 32;
    buf[offset] = 0; // verified
    offset += 1;
    buf[offset] = 100; // share
    offset += 1;

    offset
}

/// Compute the asset id from tree and nonce, matching mpl_bubblegum::utils::get_asset_id().
pub fn get_asset_id(tree: &Address, nonce: u64) -> Address {
    let nonce_bytes = nonce.to_le_bytes();
    let seeds: &[&[u8]] = &[b"asset", tree.as_ref(), &nonce_bytes];
    let (pda, _bump) =
        quasar_lang::pda::based_try_find_program_address(seeds, &crate::MPL_BUBBLEGUM_ID)
            .expect("asset PDA derivation failed");
    pda
}

/// Compute the leaf hash for a V1 LeafSchema using keccak256.
///
/// On SBF this calls the sol_keccak256 syscall directly.
/// Off-chain (tests) this is a no-op returning zeros.
pub fn leaf_schema_v1_hash(
    id: &Address,
    owner: &Address,
    delegate: &Address,
    nonce: u64,
    data_hash: &[u8; 32],
    creator_hash: &[u8; 32],
) -> [u8; 32] {
    // Input: version(1) + id(32) + owner(32) + delegate(32) + nonce(8) + data_hash(32) + creator_hash(32) = 169
    let mut input = [0u8; 169];
    input[0] = 1; // Version::V1
    input[1..33].copy_from_slice(id.as_ref());
    input[33..65].copy_from_slice(owner.as_ref());
    input[65..97].copy_from_slice(delegate.as_ref());
    input[97..105].copy_from_slice(&nonce.to_le_bytes());
    input[105..137].copy_from_slice(data_hash);
    input[137..169].copy_from_slice(creator_hash);

    keccak256(&input)
}

/// Keccak256 hash using the SBF syscall on-chain, or a placeholder off-chain.
fn keccak256(input: &[u8]) -> [u8; 32] {
    #[cfg(any(target_os = "solana", target_arch = "bpf"))]
    {
        extern "C" {
            fn sol_keccak256(vals: *const u8, val_len: u64, hash_result: *mut u8) -> u64;
        }
        let mut hash = [0u8; 32];
        // The syscall takes an array-of-slices format, same as sol_sha256.
        let input_slices: &[&[u8]] = &[input];
        unsafe {
            sol_keccak256(
                input_slices as *const _ as *const u8,
                input_slices.len() as u64,
                hash.as_mut_ptr(),
            );
        }
        hash
    }

    #[cfg(not(any(target_os = "solana", target_arch = "bpf")))]
    {
        let _ = input;
        [0u8; 32]
    }
}
