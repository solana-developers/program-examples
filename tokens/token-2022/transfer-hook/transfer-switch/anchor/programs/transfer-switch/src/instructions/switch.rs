use {
    crate::state::{AdminConfig, TransferSwitch},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct Switch<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: sender
    #[account(mut)]
    pub user: UncheckedAccount<'info>,

    /// CHECK: this account we use to take note of listings
    #[account(
        has_one=admin,
        seeds=[b"admin-config"],
        bump,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    /// CHECK: this account we use to take note of listings
    #[account(
        init_if_needed,
        payer=admin,
        space=8+TransferSwitch::INIT_SPACE,
        seeds=[user.key().as_ref()],
        bump,
    )]
    pub user_switch: Account<'info, TransferSwitch>,

    pub system_program: Program<'info, System>,
}

impl<'info> Switch<'info> {
    pub fn switch(&mut self, on: bool) -> Result<()> {
        self.user_switch.set_inner(TransferSwitch { on });
        Ok(())
    }
}
