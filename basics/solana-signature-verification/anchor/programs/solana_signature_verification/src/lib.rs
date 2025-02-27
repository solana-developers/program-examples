use anchor_lang::prelude::*;
use errors::*;
use instructions::deposit::*;
use instructions::verify_ed25519_ix::verify_ed25519_ix;
use instructions::withdraw::*;
pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("BJuWkTa4GJDn75ENWqZ18ywjRoqxmybQXupAfmSH2Zyv");
#[program]
pub mod solana_signature_verification {

    use super::*;

    // Deposit funds into the escrow account, storing the amount and unlock price
    pub fn deposit(ctx: Context<Deposit>, escrow_amt: u64, unlock_price: u64) -> Result<()> {
        let sysvar_ix = &ctx.accounts.instructions.to_account_info();
        if verify_ed25519_ix(sysvar_ix).is_ok() {
            deposit_handler(ctx, escrow_amt, unlock_price)
        } else {
            Err(SignatureVerificationError::NotSigVerified.into())
        }
    }
    // Withdraw funds from the escrow account based on Switchboard feed data
    pub fn withdraw(ctx: Context<Withdraw>, feed_id: String) -> Result<()> {
        let sysvar_ix = &ctx.accounts.instructions.to_account_info();

        if verify_ed25519_ix(sysvar_ix).is_ok() {
            withdraw_handler(ctx, feed_id)
        } else {
            Err(SignatureVerificationError::NotSigVerified.into())
        }
    }
}
