use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

pub fn pda_rent_payer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    seeds: &[&[u8]],
) -> ProgramResult {
    msg!("Executing pda_rent_payer");
    for account in accounts.iter() {
        msg!("Account: {:?}", account.key);
    }
    Ok(())
}

