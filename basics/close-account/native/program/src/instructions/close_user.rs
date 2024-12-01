use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult
};

pub fn close_user(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // Send the rent back to the payer
    **payer.lamports.borrow_mut() += target_account.lamports();
    **target_account.lamports.borrow_mut() = 0;

    // Realloc the account to zero
    target_account.realloc(0usize, true)?;

    // Assign the account to the System Program
    target_account.assign(system_program.key);

    Ok(())
}
