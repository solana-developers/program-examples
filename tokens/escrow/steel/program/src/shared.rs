use steel::*;
use spl_token::instruction::transfer;

pub fn transfer_tokens<'a, 'info>(
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    amount: u64,
    mint: &AccountInfo<'info>,
    escrow: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
) -> ProgramResult {
    // Validate accounts
    from.has_owner(&spl_token::ID)?;
    to.has_owner(&spl_token::ID)?;
    //very the mint
    mint.has_owner(&spl_token::ID)?;
    escrow.is_signer()?;
    token_program.has_address(&spl_token::ID)?;

    // Get mint data
    // let mint_data = mint.as_account::<spl_token::state::Mint>(&spl_token::ID)?;

    // Perform the transfer
    transfer(
        token_program.key,
        from.key,
        to.key,
        escrow.key,
        &[],
        amount
    )?;

    Ok(())
}