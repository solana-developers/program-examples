use {
    crate::state::Counter,
    quasar_lang::prelude::*,
};

/// Accounts for incrementing a counter.
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: &'info mut Account<Counter>,
}

impl<'info> Increment<'info> {
    #[inline(always)]
    pub fn increment(&mut self) -> Result<(), ProgramError> {
        let current: u64 = self.counter.count.into();
        self.counter.count = PodU64::from(current.checked_add(1).unwrap());
        Ok(())
    }
}
