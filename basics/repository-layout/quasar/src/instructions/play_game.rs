use quasar_lang::prelude::*;

use crate::state::game;

/// Validate game ticket requirements and log the result.
pub fn play_game(
    _name: &str,
    ticket_count: u32,
    game_name: &str,
) -> Result<(), ProgramError> {
    let games = game::get_games();

    let mut i = 0;
    while i < games.len() {
        if games[i].name_matches(game_name) {
            log("You're about to play a game!");

            if ticket_count < games[i].tickets {
                log("Sorry, you don't have enough tickets for this game!");
            } else {
                log("Let's see what you got!");
                log("Good luck winning the prize!");
            }

            return Ok(());
        }
        i += 1;
    }

    Err(ProgramError::InvalidInstructionData)
}
