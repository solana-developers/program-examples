use anchor_lang::prelude::*;

declare_id!("FcLCJkSvwQQTDfCde5LdC4DSZAqSyb2AWM9US3wF5Fp7");

#[derive(Clone)]
pub struct SPLCompression;

impl anchor_lang::Id for SPLCompression {
    fn id() -> Pubkey {
        spl_account_compression::id()
    }
}

#[program]
pub mod cnft_burn {
    use super::*;

    pub fn burn_cnft<'info>(
        ctx: Context<'_, '_, '_, 'info, BurnCnft<'info>>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
    ) -> Result<()> {
        let tree_config = ctx.accounts.tree_authority.to_account_info();
        let leaf_owner = ctx.accounts.leaf_owner.to_account_info();
        let merkle_tree = ctx.accounts.merkle_tree.to_account_info();
        let log_wrapper = ctx.accounts.log_wrapper.to_account_info();
        let compression_program = ctx.accounts.compression_program.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        let cnft_burn_cpi = mpl_bubblegum::instructions::BurnCpi::new(
            &ctx.accounts.bubblegum_program,
            mpl_bubblegum::instructions::BurnCpiAccounts {
                tree_config: &tree_config,
                leaf_owner: (&leaf_owner, true),
                leaf_delegate: (&leaf_owner, false),
                merkle_tree: &merkle_tree,
                log_wrapper: &log_wrapper,
                compression_program: &compression_program,
                system_program: &system_program,
            },
            mpl_bubblegum::instructions::BurnInstructionArgs {
                root,
                data_hash,
                creator_hash,
                nonce,
                index,
            },
        );

        cnft_burn_cpi.invoke_with_remaining_accounts(
            ctx.remaining_accounts
                .iter()
                .map(|account| (account, false, false))
                .collect::<Vec<_>>()
                .as_slice(),
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnCnft<'info> {
    #[account(mut)]
    pub leaf_owner: Signer<'info>,
    #[account(mut)]
    #[account(
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key()
    )]
    /// CHECK: This account is modified in the downstream program
    pub tree_authority: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account is neither written to nor read from.
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub log_wrapper: UncheckedAccount<'info>,
    pub compression_program: Program<'info, SPLCompression>,
    /// CHECK: This account is neither written to nor read from.
    pub bubblegum_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
