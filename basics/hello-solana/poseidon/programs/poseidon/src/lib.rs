use anchor_lang::prelude::*;

declare_id!("7fkznAWPGcMC1ck9Y1c16gMh8wqf1J4NgyJ9NFzGfRaV");

#[error_code]
pub enum PoseidonError {
    #[msg("Invalid program state")]
    InvalidState,
    #[msg("Failed to get clock")]
    ClockError,
    #[msg("Invalid instruction data")]
    InvalidInstruction,
}

#[program]
pub mod poseidon {
    use super::*;
    
    pub fn hello(ctx: Context<HelloContext>) -> Result<()> {
        msg!("Hello, Solana!");
        msg!("Program ID: {}", &id());
        
        let clock = Clock::get().map_err(|_| PoseidonError::ClockError)?;
        msg!("Timestamp: {}", clock.unix_timestamp);
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct HelloContext {}