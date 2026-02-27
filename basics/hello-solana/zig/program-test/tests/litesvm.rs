use litesvm::LiteSVM;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_native_token::LAMPORTS_PER_SOL;
use solana_pubkey::pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;

#[test]
fn test_hello_world() {
    let payer = Keypair::new();

    let mut svm = LiteSVM::new();
    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL).unwrap();

    let program_id = pubkey!("11111111111111111111111111111112");

    let program_bytes = include_bytes!("./fixtures/hello_world_program.so");
    svm.add_program(program_id, program_bytes);

    let instruction = Instruction {
        program_id,
        accounts: vec![],
        data: vec![],
    };

    let tx = Transaction::new(
        &[&payer],
        Message::new(&[instruction], Some(&payer.pubkey())),
        svm.latest_blockhash(),
    );

    let res = svm.send_transaction(tx).expect("transaction failed");
    dbg!(&res.logs);

    assert!(res.logs.iter().any(|log| log.contains("Hello, Solana!")));
    assert!(res.logs.iter().any(|log| log.contains("Our program's Program ID:")));
}
