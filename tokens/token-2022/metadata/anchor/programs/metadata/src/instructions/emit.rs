use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token_interface::{Mint, Token2022};
use spl_token_metadata_interface::instruction::emit;

#[derive(Accounts)]
pub struct Emit<'info> {
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
}

// Invoke the emit instruction from spl_token_metadata_interface directly
// There is not an anchor CpiContext for this instruction
pub fn handle_process_emit(context: Context<Emit>) -> Result<()> {
    invoke(
        &emit(
            &context.accounts.token_program.key(), // token program id
            &context.accounts.mint_account.key(),  // "metadata" account
            None,
            None,
        ),
        &[
            context.accounts.token_program.to_account_info(),
            context.accounts.mint_account.to_account_info(),
        ],
    )?;
    Ok(())
}
