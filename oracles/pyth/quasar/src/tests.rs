use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_pyth_example.so");
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), elf)
}

/// Build a minimal mock PriceUpdateV2 account body (133 bytes).
///
/// Layout:
///   [0..8]    Anchor discriminator for PriceUpdateV2
///   [8..40]   write_authority (zeroed)
///   [40]      verification_level = 1 (Full)
///   [41..73]  feed_id (0xEF * 32)
///   [73..81]  price = 15_000_000_000 i64 LE  (150.00 USD @ exponent -8)
///   [81..89]  conf = 100_000 u64 LE
///   [89..93]  exponent = -8 i32 LE
///   [93..101] publish_time = 1_700_000_000 i64 LE
///   [101..109] prev_publish_time = 1_699_999_999 i64 LE
///   [109..117] ema_price = 14_900_000_000 i64 LE
///   [117..125] ema_conf = 120_000 u64 LE
///   [125..133] posted_slot = 42 u64 LE
fn build_mock_price_update_account() -> Vec<u8> {
    let discriminator: [u8; 8] = [34, 241, 35, 99, 157, 126, 244, 205];
    let mut data = Vec::with_capacity(133);

    data.extend_from_slice(&discriminator);
    data.extend_from_slice(&[0u8; 32]); // write_authority
    data.push(1u8);                     // verification_level: Full
    data.extend_from_slice(&[0xEFu8; 32]); // feed_id
    data.extend_from_slice(&15_000_000_000i64.to_le_bytes()); // price
    data.extend_from_slice(&100_000u64.to_le_bytes());        // conf
    data.extend_from_slice(&(-8i32).to_le_bytes());           // exponent
    data.extend_from_slice(&1_700_000_000i64.to_le_bytes());  // publish_time
    data.extend_from_slice(&1_699_999_999i64.to_le_bytes());  // prev_publish_time
    data.extend_from_slice(&14_900_000_000i64.to_le_bytes()); // ema_price
    data.extend_from_slice(&120_000u64.to_le_bytes());        // ema_conf
    data.extend_from_slice(&42u64.to_le_bytes());             // posted_slot

    data
}

#[test]
fn test_read_price() {
    let mut svm = setup();

    let price_update = Pubkey::new_unique();
    let account_data = build_mock_price_update_account();

    let price_account = Account {
        address: price_update,
        lamports: 1_000_000_000,
        data: account_data,
        owner: Pubkey::new_unique(), // UncheckedAccount — no owner validation
        executable: false,
    };

    // Instruction data: discriminator = 0, no args.
    let instruction = Instruction {
        program_id: Pubkey::from(crate::ID),
        accounts: vec![solana_instruction::AccountMeta::new_readonly(
            Address::from(price_update.to_bytes()),
            false,
        )],
        data: vec![0u8],
    };

    let result = svm.process_instruction(&instruction, &[price_account]);
    result.assert_success();
}
