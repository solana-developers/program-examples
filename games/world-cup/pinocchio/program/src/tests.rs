//! Host unit tests for pure (runtime-independent) logic.

use pinocchio::Address;

use crate::{
    event_engine::{event_authority_pda, EVENT_AUTHORITY_SEED},
    state::{bracket::Bracket, common::AccountDiscriminator, config::Config, oracle::Oracle},
    tournament::{
        children, closeness, r32_slots, round_of, score_bracket, third_place_slots, validate_bracket, validate_result,
        Round, GAME_COUNT, UNDECIDED,
    },
    WorldCupError,
};

/// A "chalk" bracket where the lower-id team always advances. Champion is team 0.
fn chalk_bracket() -> [u8; GAME_COUNT] {
    let mut p = [0u8; GAME_COUNT];
    for g in 0..16u8 {
        p[g as usize] = 2 * g;
    }
    for g in 16..=30u8 {
        let (c0, c1) = children(g);
        p[g as usize] = p[c0 as usize].min(p[c1 as usize]);
    }
    let (l0, l1) = third_place_slots(&p);
    p[31] = l0.min(l1);
    p
}

#[test]
fn account_discriminator_maps_known_values_and_rejects_unknown() {
    assert_eq!(AccountDiscriminator::try_from(0u8).unwrap(), AccountDiscriminator::Config);
    assert_eq!(AccountDiscriminator::try_from(1u8).unwrap(), AccountDiscriminator::Oracle);
    assert_eq!(AccountDiscriminator::try_from(2u8).unwrap(), AccountDiscriminator::Bracket);
    assert!(AccountDiscriminator::try_from(3u8).is_err());
}

#[test]
fn account_sizes_are_fixed() {
    assert_eq!(Config::LEN, 103);
    assert_eq!(Oracle::LEN, 41);
    assert_eq!(Bracket::LEN, 78);
}

#[test]
fn round_of_partitions_all_games() {
    assert_eq!(round_of(0), Round::R32);
    assert_eq!(round_of(15), Round::R32);
    assert_eq!(round_of(16), Round::R16);
    assert_eq!(round_of(23), Round::R16);
    assert_eq!(round_of(24), Round::Qf);
    assert_eq!(round_of(27), Round::Qf);
    assert_eq!(round_of(28), Round::Sf);
    assert_eq!(round_of(29), Round::Sf);
    assert_eq!(round_of(30), Round::Final);
    assert_eq!(round_of(31), Round::ThirdPlace);
}

#[test]
fn tree_topology_matches_prd() {
    assert_eq!(children(16), (0, 1));
    assert_eq!(children(23), (14, 15));
    assert_eq!(children(24), (16, 17));
    assert_eq!(children(28), (24, 25));
    assert_eq!(children(30), (28, 29));
    assert_eq!(r32_slots(0), (0, 1));
    assert_eq!(r32_slots(15), (30, 31));
}

#[test]
fn chalk_bracket_is_consistent() {
    let p = chalk_bracket();
    assert!(validate_bracket(&p).is_ok());
    assert_eq!(p[30], 0, "chalk champion is team 0");
}

#[test]
fn rejects_pick_out_of_range() {
    let mut p = chalk_bracket();
    p[0] = 99;
    assert!(matches!(validate_bracket(&p), Err(WorldCupError::InvalidPick)));
}

#[test]
fn rejects_r32_pick_not_in_its_two_slots() {
    let mut p = chalk_bracket();
    // game 0 is contested by teams 0,1; team 5 cannot win it.
    p[0] = 5;
    assert!(matches!(validate_bracket(&p), Err(WorldCupError::InvalidPick)));
}

#[test]
fn rejects_advancing_a_team_that_was_not_advanced_from_a_feeder() {
    let mut p = chalk_bracket();
    // game 16 is fed by games 0,1 (teams 0 and 2 under chalk); team 8 never reached it.
    p[16] = 8;
    assert!(matches!(validate_bracket(&p), Err(WorldCupError::InvalidPick)));
}

#[test]
fn rejects_third_place_pick_that_is_not_a_semifinal_loser() {
    let mut p = chalk_bracket();
    let (l0, l1) = third_place_slots(&p);
    // pick a team that is neither semifinal loser.
    let bogus = (0..32u8).find(|t| *t != l0 && *t != l1).unwrap();
    p[31] = bogus;
    assert!(matches!(validate_bracket(&p), Err(WorldCupError::InvalidPick)));
}

#[test]
fn result_requires_feeders_decided_first() {
    let results = [UNDECIDED; GAME_COUNT];
    // game 16 feeds from 0,1 which are still undecided.
    assert!(matches!(validate_result(&results, 16, 0), Err(WorldCupError::FeederNotDecided)));
}

#[test]
fn result_rejects_winner_not_in_contest() {
    let results = [UNDECIDED; GAME_COUNT];
    // game 0 is teams 0,1; team 4 cannot win it.
    assert!(matches!(validate_result(&results, 0, 4), Err(WorldCupError::InvalidResult)));
}

#[test]
fn result_accepts_consistent_chain() {
    let mut results = [UNDECIDED; GAME_COUNT];
    assert!(validate_result(&results, 0, 0).is_ok());
    results[0] = 0;
    assert!(validate_result(&results, 1, 2).is_ok());
    results[1] = 2;
    // game 16 fed by 0,1 → winner must be 0 or 2.
    assert!(validate_result(&results, 16, 0).is_ok());
    assert!(matches!(validate_result(&results, 16, 1), Err(WorldCupError::InvalidResult)));
}

#[test]
fn score_is_weighted_sum_of_correct_picks() {
    let chalk = chalk_bracket();
    // Full chalk results: lower team wins every game.
    let results = chalk;
    // 16*1 + 8*2 + 4*4 + 2*8 + 1*16 + 1*8 = 16+16+16+16+16+8 = 88.
    assert_eq!(score_bracket(&chalk, &results), 88);
}

#[test]
fn score_ignores_undecided_and_wrong_picks() {
    let chalk = chalk_bracket();
    let mut results = [UNDECIDED; GAME_COUNT];
    // Only the Round of 32 is decided, all chalk → 16 points.
    results[..16].copy_from_slice(&chalk[..16]);
    assert_eq!(score_bracket(&chalk, &results), 16);

    // Flip the final's result so the champion pick is wrong: lose the 16-pt game.
    let mut full = chalk;
    let mut full_results = chalk;
    full_results[30] = 1; // some other finalist won
    full[30] = 0;
    assert_eq!(score_bracket(&full, &full_results), 88 - 16);
}

#[test]
fn closeness_is_absolute_difference() {
    assert_eq!(closeness(10, 7), 3);
    assert_eq!(closeness(7, 10), 3);
    assert_eq!(closeness(5, 5), 0);
}

#[test]
fn event_authority_pda_is_off_curve_with_valid_bump() {
    let (expected, bump) = Address::find_program_address(&[EVENT_AUTHORITY_SEED], &crate::ID);
    assert_eq!(event_authority_pda::ID, expected);
    assert_eq!(event_authority_pda::BUMP, bump);
}

#[test]
fn error_codes_are_stable() {
    assert_eq!(WorldCupError::try_from(100).unwrap() as u32, WorldCupError::NotSigner as u32);
    assert_eq!(WorldCupError::try_from(202).unwrap() as u32, WorldCupError::Unauthorized as u32);
    assert_eq!(WorldCupError::try_from(302).unwrap() as u32, WorldCupError::InvalidPick as u32);
    assert_eq!(WorldCupError::try_from(503).unwrap() as u32, WorldCupError::BracketNotBest as u32);
    assert!(WorldCupError::try_from(999).is_err());
}
