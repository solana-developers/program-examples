use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    solana_kite::{create_wallet, send_transaction_from_instructions},
    solana_signer::Signer,
};

fn setup() -> (LiteSVM, solana_keypair::Keypair) {
    let program_id = carnival::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/carnival.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, payer)
}

#[test]
fn test_go_on_rides() {
    let (mut svm, payer) = setup();

    let accounts = carnival::accounts::CarnivalContext {
        payer: payer.pubkey(),
    }
    .to_account_metas(None);

    let instructions: Vec<Instruction> = vec![
        ("Jimmy", 36u32, 15u32, "Scrambler"),
        ("Mary", 52, 1, "Ferris Wheel"),
        ("Alice", 56, 15, "Scrambler"),
        ("Bob", 49, 6, "Tilt-a-Whirl"),
    ]
    .into_iter()
    .map(|(name, height, tickets, ride)| {
        Instruction::new_with_bytes(
            carnival::id(),
            &carnival::instruction::GoOnRide {
                name: name.to_string(),
                height,
                ticket_count: tickets,
                ride_name: ride.to_string(),
            }
            .data(),
            accounts.clone(),
        )
    })
    .collect();

    send_transaction_from_instructions(&mut svm, instructions, &[&payer], &payer.pubkey()).unwrap();
}

#[test]
fn test_play_games() {
    let (mut svm, payer) = setup();

    let accounts = carnival::accounts::CarnivalContext {
        payer: payer.pubkey(),
    }
    .to_account_metas(None);

    let instructions: Vec<Instruction> = vec![
        ("Jimmy", 15u32, "I Got It!"),
        ("Mary", 1, "Ring Toss"),
        ("Alice", 15, "Ladder Climb"),
        ("Bob", 6, "Ring Toss"),
    ]
    .into_iter()
    .map(|(name, tickets, game)| {
        Instruction::new_with_bytes(
            carnival::id(),
            &carnival::instruction::PlayGame {
                name: name.to_string(),
                ticket_count: tickets,
                game_name: game.to_string(),
            }
            .data(),
            accounts.clone(),
        )
    })
    .collect();

    send_transaction_from_instructions(&mut svm, instructions, &[&payer], &payer.pubkey()).unwrap();
}

#[test]
fn test_eat_food() {
    let (mut svm, payer) = setup();

    let accounts = carnival::accounts::CarnivalContext {
        payer: payer.pubkey(),
    }
    .to_account_metas(None);

    let instructions: Vec<Instruction> = vec![
        ("Jimmy", 15u32, "Taco Shack"),
        ("Mary", 1, "Larry's Pizza"),
        ("Alice", 15, "Dough Boy's"),
        ("Bob", 6, "Dough Boy's"),
    ]
    .into_iter()
    .map(|(name, tickets, food_stand)| {
        Instruction::new_with_bytes(
            carnival::id(),
            &carnival::instruction::EatFood {
                name: name.to_string(),
                ticket_count: tickets,
                food_stand_name: food_stand.to_string(),
            }
            .data(),
            accounts.clone(),
        )
    })
    .collect();

    send_transaction_from_instructions(&mut svm, instructions, &[&payer], &payer.pubkey()).unwrap();
}
