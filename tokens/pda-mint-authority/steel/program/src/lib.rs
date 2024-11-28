use steel::*;
use api::instruction::*;

mod create_token;
mod mint_token;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction::<TokenInstruction>(&api::ID, program_id, data)?;

    match ix {
        TokenInstruction::CreateToken => create_token::process_create_token(accounts, data)?,
        TokenInstruction::MintToken => mint_token::process_mint_token(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
