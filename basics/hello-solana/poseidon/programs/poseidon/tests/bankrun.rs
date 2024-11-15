use {
    anchor_lang::{InstructionData, ToAccountMetas},
    poseidon::{self, HelloContext},
    solana_program_test::*,
    solana_sdk::{
        instruction::Instruction,
        pubkey::Pubkey,
        signature::Signer,
        signer::keypair::Keypair,
        transaction::Transaction,
    },
};

#[tokio::test]
async fn test_hello_bankrun() {
    // create program test environment
    let program_id = poseidon::ID;
    let mut program_test = ProgramTest::new(
        "poseidon",
        program_id,
        None,
    );

    // start test validator
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // create instruction
    let ix = Instruction {
        program_id,
        accounts: HelloContext {}.to_account_metas(None),
        data: poseidon::instruction::Hello {}.data(),
    };

    // create and send transaction
    let mut transaction = Transaction::new_with_payer(
        &[ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    // verify transaction execution success
    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_hello_multiple_bankrun() {
    let program_id = poseidon::ID;
    let mut program_test = ProgramTest::new(
        "poseidon",
        program_id,
        None,
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // test multiple executions
    for _ in 0..3 {
        let ix = Instruction {
            program_id,
            accounts: HelloContext {}.to_account_metas(None),
            data: poseidon::instruction::Hello {}.data(),
        };

        let mut transaction = Transaction::new_with_payer(
            &[ix],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        banks_client.process_transaction(transaction).await.unwrap();
    }
}

#[tokio::test]
async fn test_hello_error_handling() {
    let program_id = poseidon::ID;
    let mut program_test = ProgramTest::new(
        "poseidon",
        program_id,
        None,
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // test error handling
    let ix = Instruction {
        program_id,
        accounts: HelloContext {}.to_account_metas(None),
        data: poseidon::instruction::Hello {}.data(),
    };

    let mut transaction = Transaction::new_with_payer(
        &[ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_ok(), "Transaction should succeed");
}