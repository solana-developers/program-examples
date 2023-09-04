use crate::*;
use mpl_bubblegum::state::leaf_schema::LeafSchema;
use mpl_bubblegum::utils::get_asset_id;
use spl_account_compression::program::SplAccountCompression;

#[derive(Accounts)]
#[instruction(params: VerifyParams)]
pub struct Verify<'info> {
    pub leaf_owner: Signer<'info>,

    /// CHECK: This account is neither written to nor read from.
    pub leaf_delegate: AccountInfo<'info>,

    /// CHECK: unsafe
    pub merkle_tree: UncheckedAccount<'info>,

    pub compression_program: Program<'info, SplAccountCompression>,
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
        let leaf = LeafSchema::new_v0(
            asset_id,
            ctx.accounts.leaf_owner.key(),
            ctx.accounts.leaf_delegate.key(),
            params.nonce,
            params.data_hash,
            params.creator_hash,
        );

        let cpi_ctx = CpiContext::new(
            ctx.accounts.compression_program.to_account_info(),
            spl_account_compression::cpi::accounts::VerifyLeaf {
                merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
            },
        )
        .with_remaining_accounts(ctx.remaining_accounts.to_vec());

        spl_account_compression::cpi::verify_leaf(
            cpi_ctx,
            params.root,
            leaf.to_node(),
            params.index,
        )?;

        Ok(())
    }
}
