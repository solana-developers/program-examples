#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
pub mod state;
use state::PriceFeed;

pub mod error;
use error::ErrorCode;

declare_id!("F6mNuN1xoPdRaZcUX3Xviq7x1EFtoBXygpFggCLd62eU");

#[program]
pub mod pythexample {
    use super::*;
    pub fn read_price(ctx: Context<Pyth>) -> Result<()> {
        let price_feed = &ctx.accounts.price_feed;
        let clock = &ctx.accounts.clock;
        // Get the current timestamp
        let timestamp: i64 = clock.unix_timestamp;
        // Load the price from the price feed. Here, the price can be no older than 500 seconds.
        let price: pyth_sdk::Price = price_feed
            .get_price_no_older_than(timestamp, 30)
            .ok_or(ErrorCode::PythError)?;

        let confidence_interval: u64 = price.conf;

        let asset_price_full: i64 = price.price;

        let asset_exponent: i32 = price.expo;

        let asset_price = asset_price_full as f64 * 10f64.powi(asset_exponent);

        msg!("Price: {}", asset_price);
        msg!("Confidence interval: {}", confidence_interval);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Pyth<'info> {
    pub price_feed: Account<'info, PriceFeed>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}
