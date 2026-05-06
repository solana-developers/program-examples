use quasar_lang::prelude::*;

/// PDA account that tracks page visits for a user.
/// Derived from seeds: ["page_visits", user_pubkey].
#[account(discriminator = 1)]
pub struct PageVisits {
    pub page_visits: u64,
}
