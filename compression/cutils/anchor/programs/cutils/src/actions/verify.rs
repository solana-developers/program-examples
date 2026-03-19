use crate::*;
use mpl_bubblegum::types::LeafSchema;
use mpl_bubblegum::utils::get_asset_id;

#[derive(Accounts)]
#[instruction(params: VerifyParams)]
pub struct Verify<'info> {
    pub leaf_owner: Signer<'info>,

    /// CHECK: This account is neither written to nor read from.
    pub leaf_delegate: AccountInfo<'info>,

    /// CHECK: unsafe
    pub merkle_tree: UncheckedAccount<'info>,

    pub compression_program: Program<'info, SPLCompression>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct VerifyParams {
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
}

impl Verify<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &VerifyParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate<'info>(
        ctx: Context<'_, '_, '_, 'info, Verify<'info>>,
        params: &VerifyParams,
    ) -> Result<()> {
        let asset_id = get_asset_id(&ctx.accounts.merkle_tree.key(), params.nonce);
        let leaf = LeafSchema::V1 {
            id: asset_id,
            owner: ctx.accounts.leaf_owner.key(),
            delegate: ctx.accounts.leaf_delegate.key(),
            nonce: params.nonce,
            data_hash: params.data_hash,
            creator_hash: params.creator_hash,
        };

        // Build verify_leaf instruction manually because spl-account-compression 1.0.0's
        // CPI module is built against anchor-lang 0.31, which has incompatible traits with
        // anchor-lang 0.32.1. Once spl-account-compression rebuilds against 0.32.1+, replace
        // this with spl_account_compression::cpi::verify_leaf().
        use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
        use sha2::{Digest, Sha256};

        let mut accounts = vec![AccountMeta::new_readonly(
            ctx.accounts.merkle_tree.key(),
            false,
        )];
        for acc in ctx.remaining_accounts.iter() {
            accounts.push(AccountMeta::new_readonly(acc.key(), false));
        }

        // Compute the Anchor instruction discriminator: sha256("global:verify_leaf")[..8]
        let discriminator: [u8; 8] = Sha256::digest(b"global:verify_leaf")[..8]
            .try_into()
            .unwrap();
        let mut data = discriminator.to_vec();
        data.extend_from_slice(&params.root);
        data.extend_from_slice(&leaf.hash());
        data.extend_from_slice(&params.index.to_le_bytes());

        let mut account_infos = vec![ctx.accounts.merkle_tree.to_account_info()];
        for acc in ctx.remaining_accounts.iter() {
            account_infos.push(acc.to_account_info());
        }

        anchor_lang::solana_program::program::invoke(
            &Instruction {
                program_id: ctx.accounts.compression_program.key(),
                accounts,
                data,
            },
            &account_infos,
        )?;

        Ok(())
    }
}
