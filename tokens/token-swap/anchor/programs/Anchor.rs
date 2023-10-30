// Anchor code for token swapping
use anchor_lang::prelude::*;

#[program]
mod token_swapping {
    use super::*;

    pub fn swap_tokens(ctx: Context<SwapTokens>, amount: u64) -> ProgramResult {
        let sender = &mut ctx.accounts.sender;
        let recipient = &mut ctx.accounts.recipient;

        // Swap tokens from sender to recipient
        sender.token_balance -= amount;
        recipient.token_balance += amount;

        Ok(())
    }
}
