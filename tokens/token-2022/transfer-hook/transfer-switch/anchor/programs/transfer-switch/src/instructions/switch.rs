use {
    crate::state::{AdminConfig, TransferSwitch},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct Switch<'info> {
    /// admin that controls the switch
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: wallet - transfer sender
    #[account(mut)]
    pub wallet: UncheckedAccount<'info>,

    /// admin config
    #[account(
        has_one=admin,
        seeds=[b"admin-config"],
        bump,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    /// the wallet (sender) transfer switch
    #[account(
        init_if_needed,
        payer=admin,
        space=8+TransferSwitch::INIT_SPACE,
        seeds=[wallet.key().as_ref()],
        bump,
    )]
    pub wallet_switch: Account<'info, TransferSwitch>,

    pub system_program: Program<'info, System>,
}

impl<'info> Switch<'info> {
    pub fn switch(&mut self, on: bool) -> Result<()> {
        // toggle switch on/off for the given wallet
        //
        self.wallet_switch.set_inner(TransferSwitch {
            wallet: self.wallet.key(),
            on,
        });
        Ok(())
    }
}
