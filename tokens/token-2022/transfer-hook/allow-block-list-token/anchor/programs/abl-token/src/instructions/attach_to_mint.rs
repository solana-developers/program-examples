use anchor_lang::{prelude::*, solana_program::program::invoke, solana_program::system_instruction::transfer};
use anchor_spl::{
    token_2022::{spl_token_2022::{extension::{BaseStateWithExtensions, StateWithExtensions}, state::Mint as Mint2022}, Token2022},
    token_interface::{spl_token_metadata_interface::state::{Field, TokenMetadata}, token_metadata_initialize, token_metadata_update_field, Mint, TokenMetadataInitialize, TokenMetadataUpdateField},
};

use spl_tlv_account_resolution::{
     state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use crate::{Mode, META_LIST_ACCOUNT_SEED, get_extra_account_metas, get_meta_list_size};


#[derive(Accounts)]
#[instruction(args: AttachToMintArgs)]
pub struct AttachToMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint_authority: Signer<'info>,

    pub metadata_authority: Signer<'info>,

    #[account(
        mut,
        mint::token_program = token_program,
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        space = get_meta_list_size()?,
        seeds = [META_LIST_ACCOUNT_SEED, mint.key().as_ref()],
        bump,
        payer = payer,
    )]
    /// CHECK: extra metas account
    pub extra_metas_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token2022>,
}

impl AttachToMint<'_> {
    pub fn attach_to_mint(&mut self, args: AttachToMintArgs) -> Result<()> {
        let mint_info = self.mint.to_account_info();
        let mint_data = mint_info.data.borrow();
        let mint = StateWithExtensions::<Mint2022>::unpack(&mint_data)?;

        let metadata = mint.get_variable_len_extension::<TokenMetadata>();
        
        if metadata.is_err() {
            // assume metadata is not initialized, so we need to initialize it
    
            let cpi_accounts = TokenMetadataInitialize {
                program_id: self.token_program.to_account_info(),
                mint: self.mint.to_account_info(),
                metadata: self.mint.to_account_info(), // metadata account is the mint, since data is stored in mint
                mint_authority: self.mint_authority.to_account_info(),
                update_authority: self.metadata_authority.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(
                self.token_program.to_account_info(),
                cpi_accounts,
            );
            token_metadata_initialize(cpi_ctx, args.name.unwrap(), args.symbol.unwrap(), args.uri.unwrap())?;
        }

        let cpi_accounts = TokenMetadataUpdateField {
            metadata: self.mint.to_account_info(),
            update_authority: self.metadata_authority.to_account_info(),
            program_id: self.token_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            cpi_accounts,
        );

        token_metadata_update_field(cpi_ctx, Field::Key("AB".to_string()), args.mode.to_string())?;

        if args.mode == Mode::Mixed {
            let cpi_accounts = TokenMetadataUpdateField {
                metadata: self.mint.to_account_info(),
                update_authority: self.metadata_authority.to_account_info(),
                program_id: self.token_program.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(
                self.token_program.to_account_info(),
                cpi_accounts,
            );

            token_metadata_update_field(
                cpi_ctx,
                Field::Key("threshold".to_string()),
                args.threshold.to_string(),
            )?;
        }

        
        let data = self.mint.to_account_info().data_len();
        let min_balance = Rent::get()?.minimum_balance(data);
        if min_balance > self.mint.to_account_info().get_lamports() {
            invoke(
                &transfer(
                    &self.payer.key(),
                    &self.mint.to_account_info().key(),
                    min_balance - self.mint.to_account_info().get_lamports(),
                ),
                &[
                    self.payer.to_account_info(),
                    self.mint.to_account_info(),
                    self.system_program.to_account_info(),
                ],
            )?;
        }

        // initialize the extra metas account
        let extra_metas_account = &self.extra_metas_account;
        let metas = get_extra_account_metas()?;
        let mut data = extra_metas_account.try_borrow_mut_data()?;
        ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, &metas)?;

        Ok(())
        
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AttachToMintArgs {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub uri: Option<String>,
    pub mode: Mode,
    pub threshold: u64,
}

