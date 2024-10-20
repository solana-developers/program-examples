use anchor_lang::prelude::*;
declare_id!("6Ra6xuBTuL8ytoHsGEzaAQcHHqNyGjjzemHpFyYrpGdV");


#[program]
pub mod processing_instructions_program {
    use super::*;
    pub fn processing_instructions(
        ctx: Context<ProcessingInstructionsContext>,
        height: u32,
    ) -> Result<()> {

        // With Anchor, you can just put instruction data in the function signature!

        msg!("Welcome to the arena, {}!");
        if height > 8 {
            msg!("You can jump from this height length");
        } else {
            msg!("you can not jump because you are too short to handle height");
        };
        
        Ok(())
    }
}
#[derive(Accounts)]
pub struct ProcessingInstructionsContext {}
