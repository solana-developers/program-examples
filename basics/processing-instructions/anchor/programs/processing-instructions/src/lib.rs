#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("DgoL5J44aspizyUs9fcnpGEUJjWTLJRCfx8eYtUMYczf");

#[program]
pub mod processing_instructions {
    use super::*;

    // With Anchor, we just put instruction data in the function signature!
    //
    pub fn go_to_park(_ctx: Context<Park>, name: String, height: u32) -> Result<()> {
        msg!("Welcome to the park, {}!", name);
        if height > 5 {
            msg!("You are tall enough to ride this ride. Congratulations.");
        } else {
            msg!("You are NOT tall enough to ride this ride. Sorry mate.");
        };

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Park {}
