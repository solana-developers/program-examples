use litesvm::LiteSVM;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::{Keypair, Signer};
use solana_native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;

#[test]
fn test_rent() {
    let mut svm = LiteSVM::new();

    let program_id = Pubkey::new_unique();
    let program_bytes = include_bytes!("../../tests/fixtures/rent_pinocchio_program.so");

    svm.add_program(program_id, program_bytes).unwrap();

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 10).unwrap();

    let new_keypair = Keypair::new();

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(new_keypair.pubkey(), true),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
        data: [0_u8; 1000].to_vec(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &new_keypair],
        svm.latest_blockhash(),
    );

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    // rent
    let _rent = svm.get_account(&new_keypair.pubkey()).unwrap().lamports;
}
