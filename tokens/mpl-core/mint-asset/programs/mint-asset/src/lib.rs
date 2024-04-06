use anchor_lang::prelude::*;
use mpl_core::instructions::CreateV1CpiBuilder;

declare_id!("3awoVPdDiV272fh7gEBJ9etStdnSkRkNB8dx1UPpFSJE");

#[derive(Clone)]
pub struct Core;

impl anchor_lang::Id for Core {
    fn id() -> Pubkey {
        mpl_core::ID
    }
}

#[program]
pub mod mint_asset {
    use super::*;

    pub fn mint_asset(ctx: Context<MintAsset>, name: String, uri: String) -> Result<()> {
        CreateV1CpiBuilder::new(&ctx.accounts.core_program)
            .asset(&ctx.accounts.asset)
            .collection(None)
            .authority(Some(&ctx.accounts.signer))
            .payer(&ctx.accounts.signer)
            .owner(Some(&ctx.accounts.signer))
            .update_authority(Some(&ctx.accounts.signer))
            .system_program(&ctx.accounts.system_program)
            .name(name)
            .uri(uri)
            .invoke()?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintAsset<'info> {
    pub signer: Signer<'info>,

    /// CHECK: we are passing this in ourselves
    #[account(mut, signer)]
    pub asset: UncheckedAccount<'info>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}
