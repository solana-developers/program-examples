use {
    borsh::{
        BorshDeserialize, 
        BorshSerialize 
    },
    shank::ShankAccount,
    solana_program::{
        account_info::{AccountInfo, next_account_info}, 
        entrypoint::ProgramResult, 
        program::invoke_signed,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
        sysvar::Sysvar,
    },
};
use crate::state::{
    RentalOrder,
    RentalOrderStatus,
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct BookRentalArgs {
    pub name: String,
    pub pick_up_date: String,
    pub return_date: String,
    pub price: u64,
}

pub fn book_rental(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: BookRentalArgs,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let rental_order_account = next_account_info(accounts_iter)?;
    let car_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let (rental_order_account_pda, rental_order_account_bump) = RentalOrder::shank_pda(program_id, car_account.key, payer.key);
    assert!(&rental_order_account_pda == rental_order_account.key);

    let rental_order_data = RentalOrder {
        car: *car_account.key,
        name: args.name,
        pick_up_date: args.pick_up_date,
        return_date: args.return_date,
        price: args.price,
        status: RentalOrderStatus::Created,
    };

    let account_span = (rental_order_data.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    invoke_signed(
        &system_instruction::create_account(
            &payer.key,
            &rental_order_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            payer.clone(), rental_order_account.clone(), system_program.clone()
        ],
        RentalOrder::shank_seeds_with_bump(car_account.key, payer.key, &[rental_order_account_bump]),
    )?;
    
    rental_order_data.serialize(&mut &mut rental_order_account.data.borrow_mut()[..])?;

    Ok(())
}