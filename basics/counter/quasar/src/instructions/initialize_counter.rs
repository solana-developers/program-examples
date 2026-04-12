use {
    crate::state::Counter,
    quasar_lang::prelude::*,
};

/// Accounts for creating a new counter.
/// The counter is derived as a PDA from ["counter", payer] seeds.
#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: &'info mut Signer,
    #[account(mut, init, payer = payer, seeds = [b"counter", payer], bump)]
    pub counter: &'info mut Account<Counter>,
    pub system_program: &'info Program<System>,
}

impl<'info> InitializeCounter<'info> {
    #[inline(always)]
    pub fn initialize_counter(&mut self) -> Result<(), ProgramError> {
        self.counter.set_inner(0u64);
        Ok(())
    }
}
