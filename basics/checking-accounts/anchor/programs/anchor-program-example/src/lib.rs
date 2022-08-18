use anchor_lang::prelude::*;


declare_id!("ECWPhR3rJbaPfyNFgphnjxSEexbTArc7vxD8fnW6tgKw");


#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn check_accounts(_ctx: Context<CheckingAccounts>) -> Result<()> {

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckingAccounts<'info> {
    payer: Signer<'info>,
    #[account(mut)]
    /// CHECK: This account's data is empty
    account_to_create: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This account's data is empty
    account_to_change: AccountInfo<'info>,
    system_program: Program<'info, System>,
}
