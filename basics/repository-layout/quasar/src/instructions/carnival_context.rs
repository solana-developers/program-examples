use quasar_lang::prelude::*;

use super::{eat_food, get_on_ride, play_game};

/// Minimal accounts context — a signer submits the transaction.
/// The instructions just process instruction data (no on-chain state).
#[derive(Accounts)]
pub struct CarnivalContext<'info> {
    #[allow(dead_code)]
    pub payer: &'info Signer,
}

#[inline(always)]
pub fn handle_go_on_ride(
    accounts: &CarnivalContext, name: &str,
    height: u32,
    ticket_count: u32,
    ride_name: &str,
) -> Result<(), ProgramError> {
    get_on_ride::get_on_ride(name, height, ticket_count, ride_name)
}

#[inline(always)]
pub fn handle_play_game(
    accounts: &CarnivalContext, name: &str,
    ticket_count: u32,
    game_name: &str,
) -> Result<(), ProgramError> {
    play_game::play_game(name, ticket_count, game_name)
}

#[inline(always)]
pub fn handle_eat_food(
    accounts: &CarnivalContext, name: &str,
    ticket_count: u32,
    food_stand_name: &str,
) -> Result<(), ProgramError> {
    eat_food::eat_food(name, ticket_count, food_stand_name)
}
