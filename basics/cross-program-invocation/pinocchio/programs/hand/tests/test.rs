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
        include_bytes!("../../../tests/fixtures/cross_program_invocation_pinocchio_hand.so");
    let lever_program_bytes =
        include_bytes!("../../../tests/fixtures/cross_program_invocation_pinocchio_lever.so");

    let payer = Keypair::new();
    let power_account = Keypair::new();

    let mut svm = LiteSVM::new();

    svm.add_program(hand_program_id, hand_program_bytes)
        .unwrap();
    svm.add_program(lever_program_id, lever_program_bytes)
        .unwrap();

    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 10).unwrap();

    let mut data = Vec::new();
    data.push(0);
    data.push(1);

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

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let mut data = Vec::new();
    data.push(1);
    let mut name = [0u8; 8];
    let name_len = b"Chris".len().min(8);
    name[..name_len].copy_from_slice(&b"Chris"[..name_len]);
    data.extend_from_slice(&name);

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

    let res = svm.send_transaction(tx);
    dbg!(&res);
    assert!(res.is_ok());
}
