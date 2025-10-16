use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount, BaseStateWithExtensionsMut,
            PodStateWithExtensionsMut,
        },
        pod::PodAccount,
    },
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;
use std::{cell::RefMut, str::FromStr};

// transfer-hook program that charges a SOL fee on token transfer
// use a delegate and wrapped SOL because signers from initial transfer are not accessible

declare_id!("FjcHckEgXcBhFmSGai3FRpDLiT6hbpV893n8iTxVd81g");

#[error_code]
pub enum TransferError {
    #[msg("Amount Too big")]
    AmountTooBig,
    #[msg("The token is not currently transferring")]
    IsNotCurrentlyTransferring,
}

#[program]
pub mod transfer_hook {
    use super::*;

    #[interface(spl_transfer_hook_interface::initialize_extra_account_meta_list)]
    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>,
    ) -> Result<()> {
        let extra_account_metas = InitializeExtraAccountMetaList::extra_account_metas()?;

        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas,
        )?;

        Ok(())
    }

    #[interface(spl_transfer_hook_interface::execute)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        // Fail this instruction if it is not called from within a transfer hook
        check_is_transferring(&ctx)?;

        if amount > 50 {
            msg!("The amount is too big {0}", amount);
            //return err!(TransferError::AmountTooBig);
        }

        ctx.accounts.counter_account.counter += 1;

        msg!(
            "This token has been transferred {0} times",
            ctx.accounts.counter_account.counter
        );

        // All accounts are non writable so you can not burn any of them for example here
        msg!(
            "Is writable mint {0}",
            ctx.accounts.mint.to_account_info().is_writable
        );
        msg!(
            "Is destination mint {0}",
            ctx.accounts.destination_token.to_account_info().is_writable
        );
        msg!(
            "Is source mint {0}",
            ctx.accounts.source_token.to_account_info().is_writable
        );

        let signer_seeds: &[&[&[u8]]] = &[&[b"delegate", &[ctx.bumps.delegate]]];

        // Transfer WSOL from sender to delegate token account using delegate PDA
        // transfer lamports amount equal to token transfer amount
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.sender_wsol_token_account.to_account_info(),
                    mint: ctx.accounts.wsol_mint.to_account_info(),
                    to: ctx.accounts.delegate_wsol_token_account.to_account_info(),
                    authority: ctx.accounts.delegate.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            amount,
            ctx.accounts.wsol_mint.decimals,
        )?;
        Ok(())
    }
}

fn check_is_transferring(ctx: &Context<TransferHook>) -> Result<()> {
    let source_token_info = ctx.accounts.source_token.to_account_info();
    let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;
    let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
    let account_extension = account.get_extension_mut::<TransferHookAccount>()?;

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
        space = ExtraAccountMetaList::size_of(
            InitializeExtraAccountMetaList::extra_account_metas()?.len()
        )?,
        payer = payer
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(init, seeds = [b"counter"], bump, payer = payer, space = 9)]
    pub counter_account: Account<'info, CounterAccount>,
    pub system_program: Program<'info, System>,
}

// Define extra account metas to store on extra_account_meta_list account
impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        // When the token2022 program CPIs to the transfer_hook instruction on this program,
        // the accounts are provided in order defined specified the list:

        // index 0-3 are the accounts required for token transfer (source, mint, destination, owner)
        // index 4 is address of ExtraAccountMetaList account
        Ok(vec![
            // index 5, wrapped SOL mint
            ExtraAccountMeta::new_with_pubkey(
                &Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
                false,
                false,
            )?,
            // index 6, token program (for wsol token transfer)
            ExtraAccountMeta::new_with_pubkey(&Token::id(), false, false)?,
            // index 7, associated token program
            ExtraAccountMeta::new_with_pubkey(&AssociatedToken::id(), false, false)?,
            // index 8, delegate PDA
            ExtraAccountMeta::new_with_seeds(
                &[Seed::Literal {
                    bytes: b"delegate".to_vec(),
                }],
                false, // is_signer
                true,  // is_writable
            )?,
            // index 9, delegate wrapped SOL token account
            ExtraAccountMeta::new_external_pda_with_seeds(
                7, // associated token program index
                &[
                    Seed::AccountKey { index: 8 }, // owner index (delegate PDA)
                    Seed::AccountKey { index: 6 }, // token program index
                    Seed::AccountKey { index: 5 }, // wsol mint index
                ],
                false, // is_signer
                true,  // is_writable
            )?,
            // index 10, sender wrapped SOL token account
            ExtraAccountMeta::new_external_pda_with_seeds(
                7, // associated token program index
                &[
                    Seed::AccountKey { index: 3 }, // owner index
                    Seed::AccountKey { index: 6 }, // token program index
                    Seed::AccountKey { index: 5 }, // wsol mint index
                ],
                false, // is_signer
                true,  // is_writable
            )?,
            ExtraAccountMeta::new_with_seeds(
                &[Seed::Literal {
                    bytes: b"counter".to_vec(),
                }],
                false, // is_signer
                true,  // is_writable
            )?,
        ])
    }
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
    pub wsol_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(
        mut,
        seeds = [b"delegate"],
        bump
    )]
    pub delegate: SystemAccount<'info>,
    #[account(
        mut,
        token::mint = wsol_mint,
        token::authority = delegate,
    )]
    pub delegate_wsol_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = wsol_mint,
        token::authority = owner,
    )]
    pub sender_wsol_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(seeds = [b"counter"], bump)]
    pub counter_account: Account<'info, CounterAccount>,
}

#[account]
pub struct CounterAccount {
    counter: u8,
}
