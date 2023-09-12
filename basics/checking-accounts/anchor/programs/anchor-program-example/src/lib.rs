#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("ECWPhR3rJbaPfyNFgphnjxSEexbTArc7vxD8fnW6tgKw");

#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn check_accounts(_ctx: Context<CheckingAccounts>) -> Result<()> {
        Ok(())
    }
}

// Account validation in Anchor is done using the types and constraints specified in the #[derive(Accounts)] structs
// This is a simple example and does not include all possible constraints and types
#[derive(Accounts)]
pub struct CheckingAccounts<'info> {
    payer: Signer<'info>, // checks account is signer

    /// CHECK: No checks performed, example of an unchecked account
    #[account(mut)]
    account_to_create: UncheckedAccount<'info>,
    /// CHECK: Perform owner check using constraint
    #[account(
        mut,
        owner = id()
    )]
    account_to_change: UncheckedAccount<'info>,
    system_program: Program<'info, System>, // checks account is executable, and is the system program
}
