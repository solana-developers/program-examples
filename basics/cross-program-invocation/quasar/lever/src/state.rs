use quasar_lang::prelude::*;

/// On-chain power status: a single boolean toggle.
#[account(discriminator = 1)]
pub struct PowerStatus {
    pub is_on: PodBool,
}
