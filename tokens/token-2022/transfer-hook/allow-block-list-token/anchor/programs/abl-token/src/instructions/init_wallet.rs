use anchor_lang::prelude::*;

use crate::{ABWallet, Config, AB_WALLET_SEED, CONFIG_SEED};

#[derive(Accounts)]
pub struct InitWallet<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED],
        bump = config.bump,
        has_one = authority,
    )]
    pub config: Box<Account<'info, Config>>,

    pub wallet: SystemAccount<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + ABWallet::INIT_SPACE,
        seeds = [AB_WALLET_SEED, wallet.key().as_ref()],
        bump,
    )]
    pub ab_wallet: Account<'info, ABWallet>,

    pub system_program: Program<'info, System>,
}

impl InitWallet<'_> {
    pub fn init_wallet(&mut self, args: InitWalletArgs) -> Result<()> {
        let ab_wallet = &mut self.ab_wallet;
        ab_wallet.wallet = self.wallet.key();
        ab_wallet.allowed = args.allowed;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitWalletArgs {
    pub allowed: bool,
}
