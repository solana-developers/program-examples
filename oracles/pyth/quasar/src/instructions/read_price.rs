use quasar_lang::prelude::*;

/// Byte layout offsets for a Pyth PriceUpdateV2 account:
///   [0..8]    Anchor discriminator
///   [8..40]   write_authority (Pubkey)
///   [40]      verification_level (u8)
///   [41..73]  feed_id ([u8; 32])
///   [73..81]  price (i64 LE)
///   [81..89]  conf (u64 LE)
///   [89..93]  exponent (i32 LE)
///   [93..101] publish_time (i64 LE)
const PRICE_OFFSET: usize = 73;
const CONF_OFFSET: usize = 81;
const EXPONENT_OFFSET: usize = 89;
const PUBLISH_TIME_OFFSET: usize = 93;
const MIN_DATA_LEN: usize = 101;

/// Accounts for reading a Pyth PriceUpdateV2 account.
/// Uses `UncheckedAccount` because Quasar does not have a built-in Pyth account type;
/// the caller is responsible for passing a valid PriceUpdateV2 account.
#[derive(Accounts)]
pub struct ReadPrice<'info> {
    /// The Pyth PriceUpdateV2 price update account.
    pub price_update: &'info UncheckedAccount,
}

#[inline(always)]
pub fn handle_read_price(accounts: &mut ReadPrice) -> Result<(), ProgramError> {
    let view = accounts.price_update.to_account_view();
    let data = view.data();

    if data.len() < MIN_DATA_LEN {
        return Err(ProgramError::InvalidAccountData);
    }

    let price = i64::from_le_bytes(
        data[PRICE_OFFSET..PRICE_OFFSET + 8]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?,
    );
    let conf = u64::from_le_bytes(
        data[CONF_OFFSET..CONF_OFFSET + 8]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?,
    );
    let exponent = i32::from_le_bytes(
        data[EXPONENT_OFFSET..EXPONENT_OFFSET + 4]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?,
    );
    let publish_time = i64::from_le_bytes(
        data[PUBLISH_TIME_OFFSET..PUBLISH_TIME_OFFSET + 8]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?,
    );

    log("Pyth price feed data:");
    log("  price (raw):");
    log_64(price as u64);
    log("  confidence:");
    log_64(conf);
    log("  exponent:");
    log_64(exponent as u64);
    log("  publish_time:");
    log_64(publish_time as u64);

    Ok(())
}
