use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    rent::Rent,
    sysvar::Sysvar,
};

pub fn close_user(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_span = 0usize;
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    let diff = target_account.lamports() - lamports_required;

    // Send the rent back to the payer
    **target_account.lamports.borrow_mut() -= diff;
    **payer.lamports.borrow_mut() += diff;

    // Realloc the account to zero
    target_account.realloc(account_span, true)?;

    // Assign the account to the System Program
    target_account.assign(system_program.key);

    Ok(())
}
