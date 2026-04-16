use std::cell::RefMut;

use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount, BaseStateWithExtensionsMut,
            PodStateWithExtensionsMut,
        },
        pod::PodAccount,
    },
    token_interface::{Mint, TokenAccount},
};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};
use spl_discriminator::SplDiscriminate;
use spl_transfer_hook_interface::instruction::{
    ExecuteInstruction, InitializeExtraAccountMetaListInstruction,
};

declare_id!("DrWbQtYJGtsoRwzKqAbHKHKsCJJfpysudF39GBVFSxub");

#[error_code]
pub enum TransferError {
    #[msg("The token is not currently transferring")]
    IsNotCurrentlyTransferring,
}

#[program]
pub mod transfer_hook {
    use super::*;

    #[instruction(discriminator = InitializeExtraAccountMetaListInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn initialize_extra_account_meta_list(
        mut context: Context<InitializeExtraAccountMetaList>,
    ) -> Result<()> {
        // set authority field on white_list account as payer address
        context.accounts.white_list.authority = context.accounts.payer.key();

        let extra_account_metas = handle_extra_account_metas()?;

        // initialize ExtraAccountMetaList account with extra accounts
        // .map_err() needed because spl-tlv-account-resolution uses solana-program-error 2.x
        // while anchor-lang 1.0 uses 3.x — structurally identical but different semver types
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut context.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas,
        ).map_err(|_| ProgramError::InvalidAccountData)?;
        Ok(())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(context: Context<TransferHook>, _amount: u64) -> Result<()> {
        // Fail this instruction if it is not called from within a transfer hook
        check_is_transferring(&context)?;

        if !context
            .accounts
            .white_list
            .white_list
            .contains(&context.accounts.destination_token.key())
        {
            panic!("Account not in white list!");
        }

        msg!("Account in white list, all good!");

        Ok(())
    }

    pub fn add_to_whitelist(context: Context<AddToWhiteList>) -> Result<()> {
        if context.accounts.white_list.authority != context.accounts.signer.key() {
            panic!("Only the authority can add to the white list!");
        }

        context.accounts
            .white_list
            .white_list
            .push(context.accounts.new_account.key());
        msg!(
            "New account white listed! {0}",
            context.accounts.new_account.key().to_string()
        );
        msg!(
            "White list length! {0}",
            context.accounts.white_list.white_list.len()
        );

        Ok(())
    }
}

fn check_is_transferring(context: &Context<TransferHook>) -> Result<()> {
    let source_token_info = context.accounts.source_token.to_account_info();
    let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;
    // .map_err() needed because spl-token-2022 uses solana-program-error 2.x
    // while anchor-lang 1.0 uses 3.x — structurally identical but different semver types
    let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)
        .map_err(|_| ProgramError::InvalidAccountData)?;
    let account_extension = account.get_extension_mut::<TransferHookAccount>()
        .map_err(|_| ProgramError::InvalidAccountData)?;

    if !bool::from(account_extension.transferring) {
        return err!(TransferError::IsNotCurrentlyTransferring);
    }

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        // size_of returns Result with spl's ProgramError — unwrap is safe for known-good input
        space = ExtraAccountMetaList::size_of(
            handle_extra_account_metas_count()
        ).unwrap(),
        payer = payer
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    #[account(init_if_needed, seeds = [b"white_list"], bump, payer = payer, space = WhiteList::DISCRIMINATOR.len() + WhiteList::INIT_SPACE)]
    pub white_list: Account<'info, WhiteList>,
}

// Define extra account metas to store on extra_account_meta_list account
pub fn handle_extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        // .map_err() needed because spl-tlv-account-resolution uses solana-program-error 2.x
        // while anchor-lang 1.0 uses 3.x — structurally identical but different semver types
        Ok(vec![ExtraAccountMeta::new_with_seeds(
            &[Seed::Literal {
                bytes: "white_list".as_bytes().to_vec(),
            }],
            false, // is_signer
            true,  // is_writable
        ).map_err(|_| ProgramError::InvalidArgument)?])
    }



    /// Returns the count of extra account metas (avoids the error conversion issue in #[account] attributes)
pub fn handle_extra_account_metas_count() -> usize {
        1 // one extra account: the whitelist PDA
    }


// Order of accounts matters for this struct.
// The first 4 accounts are the accounts required for token transfer (source, mint, destination, owner)
// Remaining accounts are the extra accounts required from the ExtraAccountMetaList account
// These accounts are provided via CPI to this program from the token2022 program
#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(token::mint = mint, token::authority = owner)]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(token::mint = mint)]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList Account,
    #[account(seeds = [b"extra-account-metas", mint.key().as_ref()], bump)]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(seeds = [b"white_list"], bump)]
    pub white_list: Account<'info, WhiteList>,
}

#[derive(Accounts)]
pub struct AddToWhiteList<'info> {
    /// CHECK: New account to add to white list
    #[account()]
    pub new_account: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"white_list"],
        bump
    )]
    pub white_list: Account<'info, WhiteList>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct WhiteList {
    pub authority: Pubkey,
    #[max_len(11)]
    pub white_list: Vec<Pubkey>,
}
