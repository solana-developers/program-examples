use {
    borsh::{
        BorshDeserialize,
        BorshSerialize,
    },
    solana_program::{
        account_info::{AccountInfo, next_account_info}, 
        entrypoint::ProgramResult, 
        pubkey::Pubkey,
    },
};
use crate::state::{
    RentalOrder,
    RentalOrderStatus,
};

pub fn return_car(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let rental_order_account = next_account_info(accounts_iter)?;
    let car_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;

    let (rental_order_account_pda, _) = Pubkey::find_program_address(
        &[
            RentalOrder::SEED_PREFIX.as_bytes().as_ref(),
            car_account.key.as_ref(),
            payer.key.as_ref(),
        ],
        program_id,
    );
    assert!(&rental_order_account_pda == rental_order_account.key);

    let rental_order = &mut RentalOrder::try_from_slice(&rental_order_account.data.borrow())?;
    rental_order.status = RentalOrderStatus::Returned;
    rental_order.serialize(&mut &mut rental_order_account.data.borrow_mut()[..])?;

    Ok(())
}