use {crate::state::Amm, quasar_lang::prelude::*};

/// Accounts for creating a new AMM.
///
/// The Anchor version derives the AMM PDA from an `id` instruction argument.
/// In Quasar, we use a simpler fixed seed `["amm"]` since the Quasar derive
/// macro seeds reference account addresses, not instruction data.
#[derive(Accounts)]
pub struct CreateAmm<'info> {
    #[account(mut, init, payer = payer, seeds = [b"amm"], bump)]
    pub amm: &'info mut Account<Amm>,
    /// Admin authority for the AMM.
    pub admin: &'info UncheckedAccount,
    #[account(mut)]
    pub payer: &'info Signer,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_create_amm(accounts: &mut CreateAmm, id: Address, fee: u16) -> Result<(), ProgramError> {
    if fee >= 10000 {
        return Err(ProgramError::InvalidArgument);
    }
    accounts.amm.set_inner(id, *accounts.admin.address(), fee);
    Ok(())
}
