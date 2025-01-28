use anchor_lang::prelude::*;

declare_id!("5qymHa7CTb6mQkhmKpTv5rstKc4TyYcJ7hkBSi2mPnkH");
use instructions::*;
pub mod instructions;
pub mod utils;
pub mod errors;


#[program]
pub mod ed25519_example {
    use super::*;

    pub fn verify_message(
        ctx: Context<Ed25519Example>,
        message: [u8; 64],
        admin_pubkey_bytes: [u8; 32],
        signature: [u8; 64],
    ) -> Result<()> {
        ctx.accounts.verify_signature(message, admin_pubkey_bytes, signature)
    }
}
