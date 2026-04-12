use quasar_lang::prelude::*;

use crate::state::ride;

/// Validate rider requirements and log the result.
/// Quasar's `log()` takes &str — no format! in no_std — so we use static
/// messages matching the Anchor version's logic without string interpolation.
pub fn get_on_ride(
    _name: &str,
    height: u32,
    ticket_count: u32,
    ride_name: &str,
) -> Result<(), ProgramError> {
    let rides = ride::get_rides();

    let mut i = 0;
    while i < rides.len() {
        if rides[i].name_matches(ride_name) {
            log("You're about to go on a ride!");

            if ticket_count < rides[i].tickets {
                log("Sorry, you don't have enough tickets for this ride!");
                return Ok(());
            }

            if height < rides[i].min_height {
                log("Sorry, you're not tall enough for this ride!");
                return Ok(());
            }

            log("Welcome aboard the ride!");

            if rides[i].upside_down {
                log("This ride goes upside down. Hold on tight!");
            }

            return Ok(());
        }
        i += 1;
    }

    Err(ProgramError::InvalidInstructionData)
}
