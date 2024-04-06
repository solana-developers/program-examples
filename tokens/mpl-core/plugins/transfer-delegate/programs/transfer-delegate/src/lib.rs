use anchor_lang::prelude::*;
use mpl_core::{
    instructions::AddPluginV1CpiBuilder,
    types::{Plugin, PluginAuthority, TransferDelegate},
};

declare_id!("AXBiE8uaCcaw4NjZSexN5khpKEEjf47dL78btSJW4D7n");

#[derive(Clone)]
pub struct Core;

impl anchor_lang::Id for Core {
    fn id() -> Pubkey {
        mpl_core::ID
    }
}

#[program]
pub mod transfer_delegate {

    use super::*;

    // set an address as a transfer delegate
    // the transfer delegate plugin allows the delegate to transfer the asset at any time
    pub fn set_transfer_delegate(ctx: Context<TransferDelegateCtx>) -> Result<()> {
        AddPluginV1CpiBuilder::new(&ctx.accounts.core_program)
            .asset(&ctx.accounts.asset)
            .payer(&ctx.accounts.signer)
            .authority(Some(&ctx.accounts.signer))
            .system_program(&ctx.accounts.system_program)
            .plugin(Plugin::TransferDelegate(TransferDelegate {}))
            .init_authority(PluginAuthority::Address {
                address: ctx.accounts.delegate.key(),
            })
            .invoke()?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferDelegateCtx<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // this account will be able to transfer this asset on our behalf
    // this can be a PDA in which case you can pass in the Account Type
    #[account(mut)]
    pub delegate: SystemAccount<'info>,

    /// CHECK: we are passing this account ourselves
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}
