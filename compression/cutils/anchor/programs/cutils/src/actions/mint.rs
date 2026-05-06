use crate::bubblegum_types::{
    Collection, Creator, MetadataArgs, MintToCollectionV1InstructionArgs,
    TokenProgramVersion, TokenStandard, MINT_TO_COLLECTION_V1_DISCRIMINATOR,
};
use crate::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
};
use borsh::BorshSerialize;

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
    pub leaf_owner: UncheckedAccount<'info>,

    /// CHECK: This account is neither written to nor read from.
    pub leaf_delegate: UncheckedAccount<'info>,

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
    pub fn validate(&self, _context: &Context<Self>, _params: &MintParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate<'info>(
        context: Context<'info, Mint<'info>>,
        params: MintParams,
    ) -> Result<()> {
        // Build MintToCollectionV1 instruction data
        let args = MintToCollectionV1InstructionArgs {
            metadata: MetadataArgs {
                name: "BURGER".to_string(),
                symbol: "BURG".to_string(),
                uri: params.uri,
                creators: vec![Creator {
                    address: context.accounts.collection_authority.key(),
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
                    key: context.accounts.collection_mint.key(),
                }),
                token_program_version: TokenProgramVersion::Original,
                token_standard: Some(TokenStandard::NonFungible),
            },
        };

        let mut data = MINT_TO_COLLECTION_V1_DISCRIMINATOR.to_vec();
        args.serialize(&mut data)?;

        // Build account metas matching MintToCollectionV1 instruction layout
        let mut accounts = Vec::with_capacity(16);
        accounts.push(AccountMeta::new(
            context.accounts.tree_authority.key(),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.leaf_owner.key(),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.leaf_delegate.key(),
            false,
        ));
        accounts.push(AccountMeta::new(context.accounts.merkle_tree.key(), false));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.payer.key(),
            true,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.tree_delegate.key(),
            true,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.collection_authority.key(),
            true,
        ));
        // collection_authority_record_pda — pass as-is
        accounts.push(AccountMeta::new_readonly(
            context.accounts.collection_authority_record_pda.key(),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.collection_mint.key(),
            false,
        ));
        accounts.push(AccountMeta::new(
            context.accounts.collection_metadata.key(),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.edition_account.key(),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.bubblegum_signer.key(),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.log_wrapper.key(),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.compression_program.key(),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.token_metadata_program.key(),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.system_program.key(),
            false,
        ));

        let instruction = Instruction {
            program_id: MPL_BUBBLEGUM_ID,
            accounts,
            data,
        };

        // Gather all account infos for the CPI
        let account_infos = vec![
            context.accounts.bubblegum_program.to_account_info(),
            context.accounts.tree_authority.to_account_info(),
            context.accounts.leaf_owner.to_account_info(),
            context.accounts.leaf_delegate.to_account_info(),
            context.accounts.merkle_tree.to_account_info(),
            context.accounts.payer.to_account_info(),
            context.accounts.tree_delegate.to_account_info(),
            context.accounts.collection_authority.to_account_info(),
            context.accounts.collection_authority_record_pda.to_account_info(),
            context.accounts.collection_mint.to_account_info(),
            context.accounts.collection_metadata.to_account_info(),
            context.accounts.edition_account.to_account_info(),
            context.accounts.bubblegum_signer.to_account_info(),
            context.accounts.log_wrapper.to_account_info(),
            context.accounts.compression_program.to_account_info(),
            context.accounts.token_metadata_program.to_account_info(),
            context.accounts.system_program.to_account_info(),
        ];

        invoke(&instruction, &account_infos)?;

        Ok(())
    }
}
