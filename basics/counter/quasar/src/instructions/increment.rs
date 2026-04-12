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

#[inline(always)]
pub fn handle_increment(accounts: &mut Increment) -> Result<(), ProgramError> {
    let current: u64 = accounts.counter.count.into();
    accounts.counter.count = PodU64::from(current.checked_add(1).unwrap());
    Ok(())
}
