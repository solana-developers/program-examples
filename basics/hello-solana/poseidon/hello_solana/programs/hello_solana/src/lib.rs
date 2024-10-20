use anchor_lang::prelude::*;
declare_id!("Hz7UD5zsnmSwkZkareSG7pGgWLkrRe3ne5Jv1Zk1wXt5");
#[program]
pub mod hello_solana {
    use super::*;
    pub fn hello(ctx: Context<HelloContext>) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct HelloContext {}
