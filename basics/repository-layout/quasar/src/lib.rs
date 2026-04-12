#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
mod state;
#[cfg(test)]
mod tests;

declare_id!("8t94SEJh9jVjDwV7cbiuT6BvEsHo4YHP9x9a5rYH1NpP");

#[program]
mod quasar_carnival {
    use super::*;

    /// Ride a carnival ride. Validates height and ticket requirements.
    #[instruction(discriminator = 0)]
    pub fn go_on_ride(
        ctx: Ctx<CarnivalContext>,
        name: String,
        height: u32,
        ticket_count: u32,
        ride_name: String,
    ) -> Result<(), ProgramError> {
        instructions::handle_go_on_ride(&mut ctx.accounts, name, height, ticket_count, ride_name)
    }

    /// Play a carnival game. Validates ticket requirements.
    #[instruction(discriminator = 1)]
    pub fn play_game(
        ctx: Ctx<CarnivalContext>,
        name: String,
        ticket_count: u32,
        game_name: String,
    ) -> Result<(), ProgramError> {
        instructions::handle_play_game(&mut ctx.accounts, name, ticket_count, game_name)
    }

    /// Eat at a carnival food stand. Validates ticket requirements.
    #[instruction(discriminator = 2)]
    pub fn eat_food(
        ctx: Ctx<CarnivalContext>,
        name: String,
        ticket_count: u32,
        food_stand_name: String,
    ) -> Result<(), ProgramError> {
        instructions::handle_eat_food(&mut ctx.accounts, name, ticket_count, food_stand_name)
    }
}
