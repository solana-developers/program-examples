use crate::*;
use mpl_bubblegum::state::{
    metaplex_adapter::{Collection, Creator, MetadataArgs, TokenProgramVersion, TokenStandard},
    metaplex_anchor::{MplTokenMetadata, TokenMetadata},
    TreeConfig, COLLECTION_CPI_PREFIX,
};

#[derive(Accounts)]
#[instruction(params: MintParams)]
pub struct Mint<'info> {
    // #[account(
    //     init,
    //     seeds = [
    //         SEED_DATA,
    //         data.tree,
    //         data.tree_nonce
    //         // assetId directly?
    //     ],
    //     bump,
    //     payer = payer,
    //     space = Data::LEN,
    // )]
    // pub data: Account<'info, Data>,
    pub payer: Signer<'info>,

    // Bubblegum cNFT stuff MintToCollectionV1
    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        seeds::program = bubblegum_program.key(),
        bump,
    )]
    pub tree_authority: Box<Account<'info, TreeConfig>>,

    /// CHECK: This account is neither written to nor read from.
    pub leaf_owner: AccountInfo<'info>,

    /// CHECK: This account is neither written to nor read from.
    pub leaf_delegate: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: unsafe
    pub merkle_tree: UncheckedAccount<'info>,

    pub tree_delegate: Signer<'info>,

    pub collection_authority: Signer<'info>,

    /// CHECK: Optional collection authority record PDA.
    /// If there is no collecton authority record PDA then
    /// this must be the Bubblegum program address.
    pub collection_authority_record_pda: UncheckedAccount<'info>,

    /// CHECK: This account is checked in the instruction
    pub collection_mint: UncheckedAccount<'info>,

    #[account(mut)]
    pub collection_metadata: Box<Account<'info, TokenMetadata>>,

    /// CHECK: This account is checked in the instruction
    pub edition_account: UncheckedAccount<'info>,

    /// CHECK: This is just used as a signing PDA.
    #[account(
        seeds = [COLLECTION_CPI_PREFIX.as_ref()],
        seeds::program = bubblegum_program.key(),
        bump,
    )]
    pub bubblegum_signer: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub token_metadata_program: Program<'info, MplTokenMetadata>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MintParams {
    uri: String,
}

impl Mint<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &MintParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate<'info>(
        ctx: Context<'_, '_, '_, 'info, Mint<'info>>,
        params: MintParams,
    ) -> Result<()> {
        mpl_bubblegum::cpi::mint_to_collection_v1(
            CpiContext::new(
                ctx.accounts.bubblegum_program.to_account_info(),
                mpl_bubblegum::cpi::accounts::MintToCollectionV1 {
                    tree_authority: ctx.accounts.tree_authority.to_account_info(),
                    leaf_owner: ctx.accounts.leaf_owner.to_account_info(),
                    leaf_delegate: ctx.accounts.leaf_delegate.to_account_info(),
                    merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    tree_delegate: ctx.accounts.tree_delegate.to_account_info(),
                    collection_authority: ctx.accounts.collection_authority.to_account_info(),
                    collection_authority_record_pda: ctx
                        .accounts
                        .collection_authority_record_pda
                        .to_account_info(),
                    collection_mint: ctx.accounts.collection_mint.to_account_info(),
                    collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
                    edition_account: ctx.accounts.edition_account.to_account_info(),
                    bubblegum_signer: ctx.accounts.bubblegum_signer.to_account_info(),
                    log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
                    compression_program: ctx.accounts.compression_program.to_account_info(),
                    token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
            ),
            MetadataArgs {
                name: "BURGER".to_string(),
                symbol: "BURG".to_string(),
                uri: params.uri,
                creators: vec![Creator {
                    address: ctx.accounts.collection_authority.key(),
                    verified: false,
                    share: 100,
                }],
                seller_fee_basis_points: 0,
                primary_sale_happened: false,
                is_mutable: false,
                edition_nonce: Some(0),
                uses: None,
                collection: Some(Collection {
                    verified: false,
                    key: ctx.accounts.collection_mint.key(),
                }),
                token_program_version: TokenProgramVersion::Original,
                token_standard: Some(TokenStandard::NonFungible),
            },
        )?;

        Ok(())
    }
}
