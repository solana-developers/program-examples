use quasar_lang::prelude::*;

/// Message account with a dynamic-length message field.
/// Quasar's `set_inner` automatically reallocs when the new message exceeds
/// the current account size, making explicit realloc unnecessary.
#[account(discriminator = 1)]
pub struct MessageAccount<'a> {
    pub message: String,
}
