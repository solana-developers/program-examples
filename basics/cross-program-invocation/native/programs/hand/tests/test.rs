use cross_program_invocatio_native_lever::{PowerStatus, SetPowerStatus};
use litesvm::LiteSVM;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::{Keypair, Signer};
use solana_native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;

#[test]
fn test_cpi() {
    let hand_program_id = Pubkey::new_unique();
    let lever_program_id = Pubkey::new_unique();
    let hand_program_bytes =
        include_bytes!("../../../target/deploy/cross_program_invocatio_native_hand.so");
    let lever_program_bytes =
        include_bytes!("../../../target/deploy/cross_program_invocatio_native_lever.so");

    let payer = Keypair::new();
    let power_account = Keypair::new();

    let mut svm = LiteSVM::new();

    svm.add_program(hand_program_id, hand_program_bytes)
        .unwrap();
    svm.add_program(lever_program_id, lever_program_bytes)
        .unwrap();

    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 10).unwrap();

    let data = borsh::to_vec(&PowerStatus { is_on: true }).unwrap();

    let initiate_lever_ix = Instruction {
        program_id: lever_program_id,
        accounts: vec![
            AccountMeta::new(power_account.pubkey(), true),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[initiate_lever_ix],
        Some(&payer.pubkey()),
        &[&payer, &power_account],
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx).is_ok());

    let data = borsh::to_vec(&SetPowerStatus {
        name: "Chris".to_string(),
    })
    .unwrap();

    let pull_lever_ix = Instruction {
        program_id: hand_program_id,
        accounts: vec![
            AccountMeta::new(power_account.pubkey(), false),
            AccountMeta::new(lever_program_id, false),
        ],
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[pull_lever_ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx).is_ok());
}
