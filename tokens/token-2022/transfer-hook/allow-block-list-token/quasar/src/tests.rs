use crate::state::*;

/// Verify state helpers round-trip correctly.
#[test]
fn test_ab_wallet_state() {
    let addr = quasar_lang::prelude::Address::new_from_array([42u8; 32]);
    let mut buf = [0u8; AB_WALLET_SIZE as usize];

    write_ab_wallet(&mut buf, &addr, true);
    assert!(read_wallet_allowed(&buf));
    assert_eq!(read_wallet_address(&buf), &[42u8; 32]);

    write_ab_wallet(&mut buf, &addr, false);
    assert!(!read_wallet_allowed(&buf));
}

#[test]
fn test_config_state() {
    let addr = quasar_lang::prelude::Address::new_from_array([7u8; 32]);
    let mut buf = [0u8; CONFIG_SIZE as usize];

    write_config(&mut buf, &addr, 255);
    assert_eq!(read_config_authority(&buf), &[7u8; 32]);
    assert_eq!(read_config_bump(&buf), 255);
}

#[test]
fn test_mode_conversions() {
    assert_eq!(mode_to_metadata_value(0), MODE_ALLOW);
    assert_eq!(mode_to_metadata_value(1), MODE_BLOCK);
    assert_eq!(mode_to_metadata_value(2), MODE_MIXED);
    assert_eq!(mode_to_metadata_value(99), MODE_ALLOW); // default

    assert_eq!(metadata_value_to_mode(MODE_ALLOW), 0);
    assert_eq!(metadata_value_to_mode(MODE_BLOCK), 1);
    assert_eq!(metadata_value_to_mode(MODE_MIXED), 2);
    assert_eq!(metadata_value_to_mode(b"Unknown"), 0); // default
}
