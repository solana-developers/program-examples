use quasar_lang::prelude::*;

/// Minimal accounts context — a signer is needed to submit the transaction.
/// The instruction just processes instruction data (name + height).
#[derive(Accounts)]
pub struct Park<'info> {
    #[allow(dead_code)]
    pub signer: &'info Signer,
}

#[inline(always)]
pub fn handle_go_to_park(accounts: &Park, _name: &str, height: u32) -> Result<(), ProgramError> {
    // Quasar's `log()` takes &str, no format! macro available in no_std.
    // We can't interpolate the name or height into the log message, so
    // we use static messages — same logic as the Anchor version, just
    // without formatted output.
    log("Welcome to the park!");
    if height > 5 {
        log("You are tall enough to ride this ride. Congratulations.");
    } else {
        log("You are NOT tall enough to ride this ride. Sorry mate.");
    }
    Ok(())
}
