use {
    crate::{constants::MAX_FEE, errors::AmmError, state::Amm},
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program::invoke_signed,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
    },
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CreateAmmArgs {
    pub fee: u16,
}

pub fn process_create_amm(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateAmmArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let amm = next_account_info(accounts_iter)?;
    let admin = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // Check that the fee is valid
    if args.fee > MAX_FEE {
        return Err(AmmError::InvalidFee.into());
    }

    let bump = Pubkey::find_program_address(
        &[Amm::SEED_PREFIX.as_bytes(), payer.key.as_ref()],
        program_id,
    )
    .1;

    invoke_signed(
        &system_instruction::create_account(
            &payer.key,
            &amm.key,
            Rent::default().minimum_balance(Amm::space()),
            Amm::space() as u64,
            program_id,
        ),
        &[payer.clone(), amm.clone(), system_program.clone()],
        &[&[Amm::SEED_PREFIX.as_bytes(), payer.key.as_ref(), &[bump]]],
    )?;

    let amm_data = Amm {
        admin: *admin.key,
        fee: args.fee,
    };

    amm_data.serialize(&mut &mut amm.data.borrow_mut()[..])?;

    Ok(())
}
