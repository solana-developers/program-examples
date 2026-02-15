use crate::*;
use mpl_bubblegum::types::{Collection, Creator, MetadataArgs, TokenProgramVersion, TokenStandard};

#[derive(Accounts)]
#[instruction(params: MintParams)]
pub struct Mint<'info> {
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        seeds::program = bubblegum_program.key(),
        bump,
    )]
    /// CHECK: This account is modified in the downstream program
    pub tree_authority: UncheckedAccount<'info>,

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
    /// If there is no collection authority record PDA then
    /// this must be the Bubblegum program address.
    pub collection_authority_record_pda: UncheckedAccount<'info>,

    /// CHECK: This account is checked in the instruction
    pub collection_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This account is checked in the instruction
    pub collection_metadata: UncheckedAccount<'info>,

    /// CHECK: This account is checked in the instruction
    pub edition_account: UncheckedAccount<'info>,

    /// CHECK: This is just used as a signing PDA.
    pub bubblegum_signer: UncheckedAccount<'info>,

    /// CHECK: This account is neither written to nor read from.
    pub log_wrapper: UncheckedAccount<'info>,
    pub compression_program: Program<'info, SPLCompression>,
    /// CHECK: This account is neither written to nor read from.
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub bubblegum_program: UncheckedAccount<'info>,
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
        let tree_authority = ctx.accounts.tree_authority.to_account_info();
        let leaf_owner = ctx.accounts.leaf_owner.to_account_info();
        let leaf_delegate = ctx.accounts.leaf_delegate.to_account_info();
        let merkle_tree = ctx.accounts.merkle_tree.to_account_info();
        let payer = ctx.accounts.payer.to_account_info();
        let tree_delegate = ctx.accounts.tree_delegate.to_account_info();
        let collection_authority = ctx.accounts.collection_authority.to_account_info();
        let collection_authority_record_pda = ctx
            .accounts
            .collection_authority_record_pda
            .to_account_info();
        let collection_mint = ctx.accounts.collection_mint.to_account_info();
        let collection_metadata = ctx.accounts.collection_metadata.to_account_info();
        let edition_account = ctx.accounts.edition_account.to_account_info();
        let bubblegum_signer = ctx.accounts.bubblegum_signer.to_account_info();
        let log_wrapper = ctx.accounts.log_wrapper.to_account_info();
        let compression_program = ctx.accounts.compression_program.to_account_info();
        let token_metadata_program = ctx.accounts.token_metadata_program.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        let mint_cpi = mpl_bubblegum::instructions::MintToCollectionV1Cpi::new(
            &ctx.accounts.bubblegum_program,
            mpl_bubblegum::instructions::MintToCollectionV1CpiAccounts {
                tree_config: &tree_authority,
                leaf_owner: &leaf_owner,
                leaf_delegate: &leaf_delegate,
                merkle_tree: &merkle_tree,
                payer: &payer,
                tree_creator_or_delegate: &tree_delegate,
                collection_authority: &collection_authority,
                collection_authority_record_pda: Some(&collection_authority_record_pda),
                collection_mint: &collection_mint,
                collection_metadata: &collection_metadata,
                collection_edition: &edition_account,
                bubblegum_signer: &bubblegum_signer,
                log_wrapper: &log_wrapper,
                compression_program: &compression_program,
                token_metadata_program: &token_metadata_program,
                system_program: &system_program,
            },
            mpl_bubblegum::instructions::MintToCollectionV1InstructionArgs {
                metadata: MetadataArgs {
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
            },
        );

        mint_cpi.invoke()?;

        Ok(())
    }
}
