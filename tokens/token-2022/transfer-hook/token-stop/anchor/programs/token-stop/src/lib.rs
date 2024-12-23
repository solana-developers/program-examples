use anchor_lang::prelude::*;

use anchor_spl::{
    token_interface::{
        spl_token_metadata_interface::state::TokenMetadata, Mint,
        Token2022, token_metadata_update_field, TokenMetadataUpdateField
    },
    token_2022::{
        spl_token_2022::{
            extension::{
                BaseStateWithExtensions,
                StateWithExtensions,
            },
            state::Mint as Token2022Mint
        },
        ID as TOKEN_2022_PROGRAM_ID,
    },
};

use spl_tlv_account_resolution::{account::ExtraAccountMeta, state::ExtraAccountMetaList};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

declare_id!("Hy7xLUkd52wkE2xNYvNVYmmrLeqfkoqnw6sYK3cqbNqf");

#[error_code]
pub enum TransferStopError {
    #[msg("The transfers have been disabled")]
    TransfersDisabled,
}

#[program]
pub mod token_stop {
    use anchor_spl::token_interface::spl_token_metadata_interface::state::Field;

    use super::*;

    pub fn stop_transfer(ctx: Context<StopTransfer>) -> Result<()> {
        let cpi_accounts = TokenMetadataUpdateField {
            metadata: ctx.accounts.mint.to_account_info(),
            update_authority: ctx.accounts.authority.to_account_info(),
            token_program_id: ctx.accounts.token_program.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token_metadata_update_field(
            cpi_ctx,
            Field::Key("disable-transfers".to_string()),
            "true".to_string(),
        )?;

        Ok(())
    }

    pub fn resume_transfer(ctx: Context<ResumeTransfer>) -> Result<()> {
        let cpi_accounts = TokenMetadataUpdateField {
            metadata: ctx.accounts.mint.to_account_info(),
            update_authority: ctx.accounts.authority.to_account_info(),
            token_program_id: ctx.accounts.token_program.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token_metadata_update_field(
            cpi_ctx,
            Field::Key("disable-transfers".to_string()),
            "false".to_string(),
        )?;

        Ok(())
    }

    // we dont care about the token amount transferred
    #[interface(spl_transfer_hook_interface::execute)]
    pub fn execute(ctx: Context<Execute>/* , amount: u64 */) -> Result<()> {
        let mint_info = ctx.accounts.mint.to_account_info();
        let mint_data = mint_info.data.borrow();
        let mint = StateWithExtensions::<Token2022Mint>::unpack(&mint_data)?;
        
        if let Ok(metadata) = mint.get_variable_len_extension::<TokenMetadata>() {
            if let Some((_, stop_transfer_value)) = metadata.additional_metadata.iter().find(|(key, _)| key == "disable-transfers") {
                if stop_transfer_value == "true" {
                    return Err(TransferStopError::TransfersDisabled.into());
                }
            }
        }

        Ok(())
    }

    #[interface(spl_transfer_hook_interface::initialize_extra_account_meta_list)]
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let extra_metas_account = &ctx.accounts.extra_metas_account;

        let metas: Vec<ExtraAccountMeta> = vec![];
        let mut data = extra_metas_account.try_borrow_mut_data()?;
        ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, &metas)?;
        Ok(())
    } 
    
}

#[derive(Accounts)]
pub struct StopTransfer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, mint::token_program = TOKEN_2022_PROGRAM_ID)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct ResumeTransfer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, mint::token_program = TOKEN_2022_PROGRAM_ID)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token2022>,
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: This account's data is a buffer of TLV data
    #[account(
        init,
        space = ExtraAccountMetaList::size_of(0).unwrap(),
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        payer = payer,
    )]
    pub extra_metas_account: UncheckedAccount<'info>,

    #[account(
        mint::token_program = TOKEN_2022_PROGRAM_ID,
        mint::authority = mint_authority,
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub mint_authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub payer: Signer<'info>,
} 

#[derive(Accounts)]
pub struct Execute<'info> {
    /// CHECK: we don't need to check source account, hook only looks at the mint
    pub source_account: UncheckedAccount<'info>,

    pub mint: Box<InterfaceAccount<'info, Mint>>,

    // We don't need to deserialize nor check any other account, hook only looks at the mint
    // so lets save some CUs here
    //pub destination_account: Box<InterfaceAccount<'info, TokenAccount>>,
    //pub authority: UncheckedAccount<'info>,
}
