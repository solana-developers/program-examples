pub mod add_car;
pub mod book_rental;
pub mod pick_up_car;
pub mod return_car;

pub use add_car::*;
pub use book_rental::*;
pub use pick_up_car::*;
pub use return_car::*;

use {
    borsh::{
        BorshDeserialize, 
        BorshSerialize,
    },
    shank::ShankInstruction,
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankInstruction)]
pub enum CarRentalServiceInstruction {
    
    #[account(0, writable, name="car_account",
              desc="The account that will represent the Car being created")]
    #[account(1, writable, name="payer",
            desc = "Fee payer")]
    #[account(2, name="system_program",
            desc = "The System Program")]
    AddCar(AddCarArgs),

    #[account(0, writable, name="rental_account",
              desc="The account that will represent the actual order for the rental")]
    #[account(1, name="car_account",
              desc="The account representing the Car being rented in this order")]
    #[account(2, writable, name="payer",
            desc = "Fee payer")]
    #[account(3, name="system_program",
            desc = "The System Program")]
    BookRental(BookRentalArgs),

    #[account(0, writable, name="rental_account",
              desc="The account representing the active rental")]
    #[account(1, name="car_account",
              desc="The account representing the Car being rented in this order")]
    #[account(2, writable, name="payer",
            desc = "Fee payer")]
    PickUpCar,

    #[account(0, writable, name="rental_account",
              desc="The account representing the active rental")]
    #[account(1, name="car_account",
              desc="The account representing the Car being rented in this order")]
    #[account(2, writable, name="payer",
            desc = "Fee payer")]
    ReturnCar,
}