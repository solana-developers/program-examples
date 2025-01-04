// Note::
// Currently Poseidon does not support transpiling console.log to msg! calls and transpiled version of the Anchor code omits the original msg! calls
// Thus, you can add msg! statements similar to the original code to provide feedback during the program execution.
// As soon as it will be supported, the console calls will be automatically transpiled to msg! calls.

use anchor_lang::prelude::*;
declare_id!("FUfFBrs2nHAud8gVESDMtYa7oa5aGa3DEngKKLGyV2hv");
#[program]
pub mod processing_instructions_program {
    use super::*;
    pub fn go_to_park(_ctx: Context<GoToParkContext>, height: u32, name: String) -> Result<()> {
        // Note::
        // Currently Poseidon does not support transpiling console.log to msg! calls and transpiled version of the Anchor code omits the original msg! calls
        // Thus, you can add msg! statements similar to the original code to provide feedback during the program execution.
        // As soon as it will be supported, the console calls will be automatically transpiled to msg! calls.

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
pub struct GoToParkContext {}
