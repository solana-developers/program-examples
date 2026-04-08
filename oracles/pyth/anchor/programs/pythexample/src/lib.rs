use anchor_lang::prelude::*;

declare_id!("GUkjQmrLPFXXNK1bFLKt8XQi6g3TjxcHVspbjDoHvMG2");

/// Pyth Receiver program ID (rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ)
/// The pyth-solana-receiver-sdk crate depends on anchor-lang 0.32 which is
/// incompatible with Anchor 1.0. These types are inlined to avoid the conflict.
pub const PYTH_RECEIVER_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    0x04, 0xdf, 0x5d, 0xa0, 0x26, 0x6c, 0x4f, 0x33, 0x78, 0x18, 0x03, 0xbe, 0x12, 0x90, 0x4e,
    0x07, 0x58, 0x91, 0x53, 0x2a, 0x50, 0x77, 0xb9, 0xba, 0x0f, 0x18, 0x20, 0xe6, 0x2a, 0xb0,
    0x27, 0x49,
]);

/// Mirrors pyth_solana_receiver_sdk::price_update::VerificationLevel
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Debug)]
pub enum VerificationLevel {
    Partial { num_signatures: u8 },
    Full,
}

/// Mirrors pythnet_sdk::messages::PriceFeedMessage
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Debug)]
pub struct PriceFeedMessage {
    pub feed_id: [u8; 32],
    pub price: i64,
    pub conf: u64,
    pub exponent: i32,
    pub publish_time: i64,
    pub prev_publish_time: i64,
    pub ema_price: i64,
    pub ema_conf: u64,
}

/// Mirrors pyth_solana_receiver_sdk::price_update::PriceUpdateV2
/// Re-implemented here because the pyth SDK depends on anchor-lang 0.32.x,
/// which is incompatible with Anchor 1.0's types.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PriceUpdateV2 {
    pub write_authority: Pubkey,
    pub verification_level: VerificationLevel,
    pub price_message: PriceFeedMessage,
    pub posted_slot: u64,
}

/// Discriminator for PriceUpdateV2 — sha256("account:PriceUpdateV2")[..8]
/// This must match the discriminator used by pyth-solana-receiver-sdk.
pub const PRICE_UPDATE_V2_DISCRIMINATOR: [u8; 8] = {
    // Computed at compile time from sha256("account:PriceUpdateV2")
    // = [34, 241, 35, 99, 157, 126, 244, 205]
    [34, 241, 35, 99, 157, 126, 244, 205]
};

impl AccountDeserialize for PriceUpdateV2 {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        // Skip 8-byte discriminator
        if buf.len() < 8 {
            return Err(ErrorCode::AccountDidNotDeserialize.into());
        }
        *buf = &buf[8..];
        AnchorDeserialize::deserialize(buf).map_err(|_| ErrorCode::AccountDidNotDeserialize.into())
    }

    fn try_deserialize(buf: &mut &[u8]) -> Result<Self> {
        if buf.len() < 8 {
            return Err(ErrorCode::AccountDidNotDeserialize.into());
        }
        let disc = &buf[..8];
        if disc != PRICE_UPDATE_V2_DISCRIMINATOR {
            return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        }
        Self::try_deserialize_unchecked(buf)
    }
}

impl AccountSerialize for PriceUpdateV2 {}

impl Owner for PriceUpdateV2 {
    fn owner() -> Pubkey {
        PYTH_RECEIVER_PROGRAM_ID
    }
}

impl Discriminator for PriceUpdateV2 {
    const DISCRIMINATOR: &'static [u8] = &PRICE_UPDATE_V2_DISCRIMINATOR;
}

#[program]
pub mod anchor_test {
    use super::*;

    pub fn read_price(ctx: Context<ReadPrice>) -> Result<()> {
        let price_update = &ctx.accounts.price_update;
        msg!("Price feed id: {:?}", price_update.price_message.feed_id);
        msg!("Price: {:?}", price_update.price_message.price);
        msg!("Confidence: {:?}", price_update.price_message.conf);
        msg!("Exponent: {:?}", price_update.price_message.exponent);
        msg!("Publish Time: {:?}", price_update.price_message.publish_time);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReadPrice<'info> {
    /// A PriceUpdateV2 account owned by the Pyth Receiver program.
    pub price_update: Account<'info, PriceUpdateV2>,
}
