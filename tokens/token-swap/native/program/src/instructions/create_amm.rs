use {
    crate::{constants::MAX_FEE, errors::AmmError, state::Amm},
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program::invoke_signed,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
    },
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateAmmArgs {
    pub fee: u16,
}

pub fn create_amm(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateAmmArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let amm_account = next_account_info(accounts_iter)?;
    let admin_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // Check that the fee is valid
    if args.fee > MAX_FEE {
        return Err(AmmError::InvalidFee.into());
    }

    // Check that amm_account is correct
    let (pda, bump) = Pubkey::find_program_address(
        &[Amm::SEED_PREFIX.as_bytes(), payer.key.as_ref()],
        program_id,
    );
    if &pda != amm_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    invoke_signed(
        &system_instruction::create_account(
            &payer.key,
            &amm_account.key,
            Rent::default().minimum_balance(Amm::space()),
            Amm::space() as u64,
            program_id,
        ),
        &[payer.clone(), amm_account.clone(), system_program.clone()],
        &[&[Amm::SEED_PREFIX.as_bytes(), payer.key.as_ref(), &[bump]]],
    )?;

    let amm_data = Amm {
        admin: *admin_account.key,
        fee: args.fee,
    };

    amm_data.serialize(&mut &mut amm_account.data.borrow_mut()[..])?;

    Ok(())
}
