use {
    crate::state::AdminConfig,
    anchor_lang::{
        prelude::*,
        system_program::{create_account, CreateAccount},
    },
    anchor_spl::token_interface::Mint,
    spl_tlv_account_resolution::{
        account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
    },
    spl_transfer_hook_interface::instruction::ExecuteInstruction,
};

#[derive(Accounts)]
// #[instruction(decimals: u8)]
pub struct InitializeExtraAccountMetas<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account()]
    pub token_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: this account we use to take note of listings
    #[account(
        init,
        payer=admin,
        space=8+AdminConfig::INIT_SPACE,
        seeds=[b"admin-config"],
        bump
    )]
    pub admin_config: Account<'info, AdminConfig>,

    /// CHECK: extra accoumt metas list
    #[account(
        mut,
        seeds = [b"extra-account-metas", token_mint.key().as_ref()],
        bump,
    )]
    pub extra_account_metas_list: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetas<'info> {
    pub fn init_admin_config(&mut self) -> Result<()> {
        self.admin_config.set_inner(AdminConfig {
            admin: self.admin.key(), // set the admin pubkey that can switch transfers on/off
        });
        Ok(())
    }

    pub fn initialize_extra_account_metas_list(
        &self,
        bumps: InitializeExtraAccountMetasBumps,
    ) -> Result<()> {
        let account_metas = vec![
            // 5 - user (sender) config account
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::AccountKey { index: 3 }, // sender index
                ],
                false, // is_signer
                true,  // is_writable
            )?,
        ];

        // calculate account size
        let account_size = ExtraAccountMetaList::size_of(account_metas.len())? as u64;

        // calculate minimum required lamports
        let lamports = Rent::get()?.minimum_balance(account_size as usize);

        let mint = self.token_mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"extra-account-metas",
            mint.as_ref(),
            &[bumps.extra_account_metas_list],
        ]];

        create_account(
            CpiContext::new(
                self.system_program.to_account_info(),
                CreateAccount {
                    from: self.admin.to_account_info(),
                    to: self.extra_account_metas_list.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            lamports,
            account_size,
            &crate::ID,
        )?;

        // Initialize the account data to store the list of ExtraAccountMetas
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut self.extra_account_metas_list.try_borrow_mut_data()?,
            &account_metas,
        )?;

        Ok(())
    }
}
