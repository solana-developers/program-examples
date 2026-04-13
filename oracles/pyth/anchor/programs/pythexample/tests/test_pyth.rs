use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{create_wallet, send_transaction_from_instructions},
    solana_signer::Signer,
};

/// Pyth Receiver program ID (rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ)
fn pyth_receiver_program_id() -> anchor_lang::solana_program::pubkey::Pubkey {
    pythexample::PYTH_RECEIVER_PROGRAM_ID
}

/// Build mock PriceUpdateV2 account data with Anchor discriminator.
fn build_mock_price_update_account(
    write_authority: &anchor_lang::solana_program::pubkey::Pubkey,
) -> Vec<u8> {
    // Discriminator: sha256("account:PriceUpdateV2")[..8]
    let discriminator: [u8; 8] = [34, 241, 35, 99, 157, 126, 244, 205];

    let mut data = Vec::with_capacity(133);

    // Discriminator
    data.extend_from_slice(&discriminator);

    // write_authority: Pubkey (32 bytes)
    data.extend_from_slice(write_authority.as_ref());

    // verification_level: Full = borsh enum variant 1
    data.push(1u8);

    // PriceFeedMessage fields:
    // feed_id: [u8; 32]
    let feed_id = [0xEFu8; 32];
    data.extend_from_slice(&feed_id);

    // price: i64 (150.00000000 USD with exponent -8)
    let price: i64 = 15_000_000_000;
    data.extend_from_slice(&price.to_le_bytes());

    // conf: u64
    let conf: u64 = 100_000;
    data.extend_from_slice(&conf.to_le_bytes());

    // exponent: i32
    let exponent: i32 = -8;
    data.extend_from_slice(&exponent.to_le_bytes());

    // publish_time: i64
    let publish_time: i64 = 1_700_000_000;
    data.extend_from_slice(&publish_time.to_le_bytes());

    // prev_publish_time: i64
    let prev_publish_time: i64 = 1_699_999_999;
    data.extend_from_slice(&prev_publish_time.to_le_bytes());

    // ema_price: i64
    let ema_price: i64 = 14_900_000_000;
    data.extend_from_slice(&ema_price.to_le_bytes());

    // ema_conf: u64
    let ema_conf: u64 = 120_000;
    data.extend_from_slice(&ema_conf.to_le_bytes());

    // posted_slot: u64
    let posted_slot: u64 = 42;
    data.extend_from_slice(&posted_slot.to_le_bytes());

    data
}

#[test]
fn test_read_price() {
    let program_id = pythexample::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/pythexample.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();

    // Create a mock PriceUpdateV2 account
    let price_update_key = Keypair::new();
    let account_data = build_mock_price_update_account(&payer.pubkey());

    // Set the account in LiteSVM with the Pyth Receiver program as owner
    let pyth_receiver_id = pyth_receiver_program_id();
    let rent = svm.minimum_balance_for_rent_exemption(account_data.len());

    svm.set_account(
        price_update_key.pubkey(),
        solana_account::Account {
            lamports: rent,
            data: account_data,
            owner: pyth_receiver_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();

    // Call read_price — program just reads the account and logs the price info
    let ix_data = pythexample::instruction::ReadPrice {}.data();

    let accounts = pythexample::accounts::ReadPrice {
        price_update: price_update_key.pubkey(),
    }
    .to_account_metas(None);

    let instruction = Instruction::new_with_bytes(program_id, &ix_data, accounts);

    send_transaction_from_instructions(&mut svm, vec![instruction], &[&payer], &payer.pubkey())
        .unwrap();
}
