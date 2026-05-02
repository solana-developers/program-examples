use quasar_lang::prelude::*;

use crate::state::food;

/// Validate food stand ticket requirements and log the result.
pub fn eat_food(
    _name: &str,
    ticket_count: u32,
    food_stand_name: &str,
) -> Result<(), ProgramError> {
    let stands = food::get_food_stands();

    let mut i = 0;
    while i < stands.len() {
        if stands[i].name_matches(food_stand_name) {
            log("Welcome to the food stand!");

            if ticket_count < stands[i].tickets {
                log("Sorry, you don't have enough tickets for this food!");
            } else {
                log("Enjoy your food!");
            }

            return Ok(());
        }
        i += 1;
    }

    Err(ProgramError::InvalidInstructionData)
}
