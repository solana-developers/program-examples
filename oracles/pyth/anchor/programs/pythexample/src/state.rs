// This file is the pyth default implementation for using PriceAccount in your Anchor context.
use anchor_lang::prelude::*;
use pyth_sdk_solana::state::load_price_account;
use std::ops::Deref;
use std::str::FromStr;

// import the error code from the error.rs file
use crate::error::ErrorCode;

#[derive(Clone)]
pub struct PriceFeed(pyth_sdk::PriceFeed);

impl anchor_lang::Owner for PriceFeed {
    fn owner() -> Pubkey {
        // The mainnet Pyth program ID
        let oracle_addr = "FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH";
        Pubkey::from_str(oracle_addr).unwrap()
    }
}

impl anchor_lang::AccountDeserialize for PriceFeed {
    fn try_deserialize_unchecked(data: &mut &[u8]) -> Result<Self> {
        let account = load_price_account(data).map_err(|_x| error!(ErrorCode::PythError))?;
        let zeros: [u8; 32] = [0; 32];
        let dummy_key = Pubkey::new_from_array(zeros);
        let feed = account.to_price_feed(&dummy_key);
        Ok(PriceFeed(feed))
    }
}

impl anchor_lang::AccountSerialize for PriceFeed {
    fn try_serialize<W: std::io::Write>(&self, _writer: &mut W) -> std::result::Result<(), Error> {
        Err(error!(ErrorCode::TryToSerializePriceAccount))
    }
}

impl Deref for PriceFeed {
    type Target = pyth_sdk::PriceFeed;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
