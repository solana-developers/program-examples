use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_carnival.so");
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), elf)
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 10_000_000_000)
}

/// Build go_on_ride instruction data (discriminator = 0).
/// Wire format: [disc=0] [ZC: height(u32), ticket_count(u32)] [name: String] [ride_name: String]
fn build_go_on_ride(name: &str, height: u32, ticket_count: u32, ride_name: &str) -> Vec<u8> {
    let mut data = vec![0u8]; // discriminator = 0

    // Fixed ZC fields: height, ticket_count
    data.extend_from_slice(&height.to_le_bytes());
    data.extend_from_slice(&ticket_count.to_le_bytes());

    // Dynamic String: name
    data.extend_from_slice(&(name.len() as u32).to_le_bytes());
    data.extend_from_slice(name.as_bytes());

    // Dynamic String: ride_name
    data.extend_from_slice(&(ride_name.len() as u32).to_le_bytes());
    data.extend_from_slice(ride_name.as_bytes());

    data
}

/// Build play_game instruction data (discriminator = 1).
/// Wire format: [disc=1] [ZC: ticket_count(u32)] [name: String] [game_name: String]
fn build_play_game(name: &str, ticket_count: u32, game_name: &str) -> Vec<u8> {
    let mut data = vec![1u8]; // discriminator = 1

    // Fixed ZC: ticket_count
    data.extend_from_slice(&ticket_count.to_le_bytes());

    // Dynamic String: name
    data.extend_from_slice(&(name.len() as u32).to_le_bytes());
    data.extend_from_slice(name.as_bytes());

    // Dynamic String: game_name
    data.extend_from_slice(&(game_name.len() as u32).to_le_bytes());
    data.extend_from_slice(game_name.as_bytes());

    data
}

/// Build eat_food instruction data (discriminator = 2).
/// Wire format: [disc=2] [ZC: ticket_count(u32)] [name: String] [food_stand_name: String]
fn build_eat_food(name: &str, ticket_count: u32, food_stand_name: &str) -> Vec<u8> {
    let mut data = vec![2u8]; // discriminator = 2

    // Fixed ZC: ticket_count
    data.extend_from_slice(&ticket_count.to_le_bytes());

    // Dynamic String: name
    data.extend_from_slice(&(name.len() as u32).to_le_bytes());
    data.extend_from_slice(name.as_bytes());

    // Dynamic String: food_stand_name
    data.extend_from_slice(&(food_stand_name.len() as u32).to_le_bytes());
    data.extend_from_slice(food_stand_name.as_bytes());

    data
}

fn make_ix(data: Vec<u8>, user: Pubkey) -> Instruction {
    Instruction {
        program_id: Pubkey::from(crate::ID),
        accounts: vec![
            solana_instruction::AccountMeta::new_readonly(
                Address::from(user.to_bytes()),
                true,
            ),
        ],
        data,
    }
}

#[test]
fn test_go_on_ride_success() {
    let mut svm = setup();
    let user = Pubkey::new_unique();
    let data = build_go_on_ride("Alice", 60, 5, "Ferris Wheel");
    let ix = make_ix(data, user);

    let result = svm.process_instruction(&ix, &[signer(user)]);
    result.assert_success();

    let logs = result.logs.join("\n");
    assert!(logs.contains("about to go on a ride"), "should announce ride");
    assert!(logs.contains("Welcome aboard"), "should welcome aboard");
}

#[test]
fn test_go_on_ride_not_tall_enough() {
    let mut svm = setup();
    let user = Pubkey::new_unique();
    let data = build_go_on_ride("Bob", 40, 5, "Ferris Wheel");
    let ix = make_ix(data, user);

    let result = svm.process_instruction(&ix, &[signer(user)]);
    result.assert_success();

    let logs = result.logs.join("\n");
    assert!(logs.contains("not tall enough"), "should reject short rider");
}

#[test]
fn test_go_on_ride_not_enough_tickets() {
    let mut svm = setup();
    let user = Pubkey::new_unique();
    let data = build_go_on_ride("Charlie", 60, 1, "Zero Gravity");
    let ix = make_ix(data, user);

    let result = svm.process_instruction(&ix, &[signer(user)]);
    result.assert_success();

    let logs = result.logs.join("\n");
    assert!(logs.contains("enough tickets"), "should reject insufficient tickets");
}

#[test]
fn test_go_on_ride_upside_down() {
    let mut svm = setup();
    let user = Pubkey::new_unique();
    let data = build_go_on_ride("Dave", 65, 5, "Zero Gravity");
    let ix = make_ix(data, user);

    let result = svm.process_instruction(&ix, &[signer(user)]);
    result.assert_success();

    let logs = result.logs.join("\n");
    assert!(logs.contains("upside down"), "should warn about upside down");
}

#[test]
fn test_play_game_success() {
    let mut svm = setup();
    let user = Pubkey::new_unique();
    let data = build_play_game("Alice", 5, "Ring Toss");
    let ix = make_ix(data, user);

    let result = svm.process_instruction(&ix, &[signer(user)]);
    result.assert_success();

    let logs = result.logs.join("\n");
    assert!(logs.contains("about to play"), "should announce game");
    assert!(logs.contains("what you got"), "should encourage player");
}

#[test]
fn test_play_game_not_enough_tickets() {
    let mut svm = setup();
    let user = Pubkey::new_unique();
    let data = build_play_game("Bob", 1, "Ring Toss");
    let ix = make_ix(data, user);

    let result = svm.process_instruction(&ix, &[signer(user)]);
    result.assert_success();

    let logs = result.logs.join("\n");
    assert!(logs.contains("enough tickets"), "should reject insufficient tickets");
}

#[test]
fn test_eat_food_success() {
    let mut svm = setup();
    let user = Pubkey::new_unique();
    let data = build_eat_food("Alice", 3, "Larry's Pizza");
    let ix = make_ix(data, user);

    let result = svm.process_instruction(&ix, &[signer(user)]);
    result.assert_success();

    let logs = result.logs.join("\n");
    assert!(logs.contains("food stand"), "should welcome to food stand");
    assert!(logs.contains("Enjoy"), "should say enjoy");
}

#[test]
fn test_eat_food_not_enough_tickets() {
    let mut svm = setup();
    let user = Pubkey::new_unique();
    let data = build_eat_food("Bob", 0, "Larry's Pizza");
    let ix = make_ix(data, user);

    let result = svm.process_instruction(&ix, &[signer(user)]);
    result.assert_success();

    let logs = result.logs.join("\n");
    assert!(logs.contains("enough tickets"), "should reject insufficient tickets");
}

#[test]
fn test_invalid_ride_name() {
    let mut svm = setup();
    let user = Pubkey::new_unique();
    let data = build_go_on_ride("Eve", 60, 5, "Nonexistent Ride");
    let ix = make_ix(data, user);

    let result = svm.process_instruction(&ix, &[signer(user)]);
    assert!(result.raw_result.is_err(), "should fail for unknown ride");
}
