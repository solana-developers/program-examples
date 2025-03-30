// This file is the pyth default implementation for using PriceAccount in your Anchor context.
use anchor_lang::prelude::*;
use pyth_sdk_solana::state::load_price_account;
use std::ops::Deref;
use pyth_sdk_solana::state::SolanaPriceAccount;

// import the error code from the error.rs file
use crate::error::ErrorCode;

#[derive(Clone)]
pub struct PriceFeed(pyth_sdk::PriceFeed);

#[cfg(feature = "idl-build")]
impl anchor_lang::IdlBuild for PriceFeed {}

#[cfg(feature = "idl-build")]
impl anchor_lang::Discriminator for PriceFeed {
    const DISCRIMINATOR: &'static [u8] = &[];
}

const PYTH_PROGRAM_ID: [Pubkey; 1] = [Pubkey::from_str_const(
    "FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH",
)];

impl anchor_lang::Owners for PriceFeed {
    fn owners() -> &'static [Pubkey] {
        &PYTH_PROGRAM_ID
    }
}

impl anchor_lang::AccountDeserialize for PriceFeed {
    fn try_deserialize_unchecked(data: &mut &[u8]) -> Result<Self> {
        let account: &SolanaPriceAccount =
            load_price_account(data).map_err(|_x| {
                error!(ErrorCode::PythError)
        })?;
        let zeros: [u8; 32] = [0; 32];
        let dummy_key = Pubkey::new_from_array(zeros);
        let feed = account.to_price_feed(&dummy_key);
        Ok(PriceFeed(feed))
    }
}

impl anchor_lang::AccountSerialize for PriceFeed {}

impl Deref for PriceFeed {
    type Target = pyth_sdk::PriceFeed;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
