use hello_solana_api::prelude::*;
use steel::*;
use solana_program::{msg};

pub fn process_hello(accounts: &[AccountInfo<'_>]) -> ProgramResult {
    let [signer_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    msg!("Hello, Solana!");

    msg!("Our program's Program ID: {}", &hello_solana_api::ID);

    Ok(())
}
