use anchor_lang::{prelude::*, solana_program::sysvar::instructions::ID as INSTRUCTIONS_ID};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::{
            instructions::{CreateV1Cpi, CreateV1CpiAccounts, CreateV1InstructionArgs},
            types::{Collection, Creator, PrintSupply, TokenStandard},
        },
        Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub master_edition: UncheckedAccount<'info>,
    #[account(
        seeds = [b"authority"],
        bump,
    )]
    /// CHECK: This is account is not initialized and is being used for signing purposes only
    pub mint_authority: UncheckedAccount<'info>,
    #[account(address = INSTRUCTIONS_ID)]
    /// CHECK: Sysvar instruction account that is being checked with an address constraint
    sysvar_instructions: UncheckedAccount<'info>,
    #[account(mut)]
    pub collection_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
}

impl<'info> MintNFT<'info> {
    pub fn mint_nft(&mut self, bumps: &MintNFTBumps) -> Result<()> {
        let metadata = &self.metadata.to_account_info();
        let master_edition = &self.master_edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let authority = &self.mint_authority.to_account_info();
        let payer = &self.owner.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let spl_token_program = &self.token_program.to_account_info();
        let spl_metadata_program = &self.token_metadata_program.to_account_info();
        let sysvar_instructions = &self.sysvar_instructions.to_account_info();

        let seeds = &[&b"authority"[..], &[bumps.mint_authority]];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        mint_to(cpi_ctx, 1)?;
        msg!("Collection NFT minted!");

        let creator = vec![Creator {
            address: self.mint_authority.key(),
            verified: true,
            share: 100,
        }];

        let create_v1_cpi = CreateV1Cpi::new(
            spl_metadata_program,
            CreateV1CpiAccounts {
                metadata,
                master_edition: Some(master_edition),
                mint: (mint, true),
                authority,
                payer,
                update_authority: (authority, true),
                system_program,
                sysvar_instructions,
                spl_token_program: Some(spl_token_program),
            },
            CreateV1InstructionArgs {
                name: "Mint Test".to_string(),
                symbol: "YAY".to_string(),
                uri: "".to_string(),
                seller_fee_basis_points: 0,
                creators: Some(creator),
                collection: Some(Collection {
                    verified: false,
                    key: self.collection_mint.key(),
                }),
                uses: None,
                primary_sale_happened: false,
                print_supply: Some(PrintSupply::Zero),
                is_mutable: true,
                token_standard: TokenStandard::NonFungible,
                collection_details: None,
                rule_set: None,
                decimals: Some(0),
            },
        );

        create_v1_cpi.invoke_signed(signer_seeds)?;

        msg!("Metadata Account and Master Edition Account created!");

        Ok(())
    }
}
