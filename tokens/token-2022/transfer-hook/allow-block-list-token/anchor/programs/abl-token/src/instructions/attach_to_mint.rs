use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{transfer_hook_update, Mint, TransferHookUpdate},
};

use spl_tlv_account_resolution::state::ExtraAccountMetaList;
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use crate::{get_extra_account_metas, get_meta_list_size, META_LIST_ACCOUNT_SEED};

#[derive(Accounts)]
pub struct AttachToMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

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
    pub fn attach_to_mint(&mut self) -> Result<()> {
        let tx_hook_accs = TransferHookUpdate {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.payer.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), tx_hook_accs);

        transfer_hook_update(ctx, Some(crate::ID_CONST))?;

        // initialize the extra metas account
        let extra_metas_account = &self.extra_metas_account;
        let metas = get_extra_account_metas()?;
        let mut data = extra_metas_account.try_borrow_mut_data()?;
        ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, &metas)?;

        Ok(())
    }
}
