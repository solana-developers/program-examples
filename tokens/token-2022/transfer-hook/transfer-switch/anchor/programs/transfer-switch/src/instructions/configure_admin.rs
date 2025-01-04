use {crate::state::AdminConfig, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct ConfigureAdmin<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: the new admin
    #[account(mut)]
    pub new_admin: UncheckedAccount<'info>,

    /// To hold the address of the admin that controls switches
    #[account(
        init_if_needed,
        payer=admin,
        space=8+AdminConfig::INIT_SPACE,
        seeds=[b"admin-config"],
        bump
    )]
    pub admin_config: Account<'info, AdminConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> ConfigureAdmin<'info> {
    pub fn is_admin(&self) -> Result<()> {
        // check if we are not creating the account for the first time,
        // ensure it's the admin that is making the change
        //
        if self.admin_config.is_initialised {
            // make sure it's the admin
            //
            require_keys_eq!(self.admin.key(), self.admin_config.admin,);

            // make sure the admin is not reentering their key
            //
            require_keys_neq!(self.admin.key(), self.new_admin.key());
        }
        Ok(())
    }

    pub fn configure_admin(&mut self) -> Result<()> {
        self.admin_config.set_inner(AdminConfig {
            admin: self.new_admin.key(), // set the admin pubkey that can switch transfers on/off
            is_initialised: true,        // let us know an admin has been set
        });
        Ok(())
    }
}
