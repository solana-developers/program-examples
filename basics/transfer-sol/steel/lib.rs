use steel::prelude::*;
use solana_program::{
    system_instruction,
    program::invoke,
    pubkey::Pubkey,
};

#[program]
mod transfer_sol_program {
    use super::*;

    pub fn transfer_sol(ctx: Context<TransferSol>, lamports: u64) -> ProgramResult {
        let ix = system_instruction::transfer(ctx.accounts.sender.key, ctx.accounts.receiver.key, lamports);
        invoke(&ix, &[ctx.accounts.sender.clone(), ctx.accounts.receiver.clone()])
    }
}

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut, signer)]
    pub sender: AccountInfo<'info>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
}
