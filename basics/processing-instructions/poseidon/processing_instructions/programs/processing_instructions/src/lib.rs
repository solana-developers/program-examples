use anchor_lang::prelude::*;
declare_id!("ESFoo5N4Zv65pph6thqP3HVqiY7KH5o5V8TqTsnmB2vw");
#[program]
pub mod processing_instructions {
    use super::*;
    pub fn go_to_park(ctx: Context<GoToParkContext>, name: String) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct GoToParkContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}
