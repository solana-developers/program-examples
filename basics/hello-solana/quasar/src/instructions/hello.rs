use quasar_lang::prelude::*;

/// Accounts for the hello instruction.
/// A payer (signer) is required to submit the transaction, but the program
/// simply logs a greeting and the program ID.
#[derive(Accounts)]
pub struct Hello<'info> {
    #[allow(dead_code)]
    pub payer: &'info Signer,
}

#[inline(always)]
pub fn handle_hello(accounts: &Hello) -> Result<(), ProgramError> {
    log("Hello, Solana!");
    log("Our program's Program ID: FLUH9c5oAfXb1eYbkZvdGK9r9SLQJBUi2DZQaBVj7Tzr");
    Ok(())
}
