use solana_program::{msg, program_pack::Pack};
use spl_token::state::Mint;
use steel::*;
use steel_api::prelude::*;

pub fn process_create_token(accounts: &[AccountInfo], name: String, symbol: String) -> ProgramResult {
    let mint = Mint {
        mint_authority: COption::Some(accounts[0].key.to_owned()),
        supply: 0,
        decimals: 2,
        is_initialized: false,
        freeze_authority: COption::None,
    };

    let mut data = vec![];
    mint.pack_into_slice(&mut data);

    let mut instruction = Instruction::new(
        accounts[0].key.to_owned(),
        vec![],
        vec![AccountMeta::new_readonly(accounts[0].key.to_owned(), false)],
        data,
    );

    let mut instruction_data = vec![];
    instruction.pack_into_slice(&mut instruction_data);

    msg!("Instruction data: {:?}", instruction_data);

    Ok(())
}
