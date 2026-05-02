use quasar_lang::prelude::*;

/// On-chain counter account.
#[account(discriminator = 1)]
pub struct Counter {
    pub count: u64,
}
