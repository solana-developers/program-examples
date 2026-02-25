use anchor_lang::prelude::*;
use chainlink_solana::v2::read_feed_v2;

//Program ID required by Anchor. Replace with your unique program ID once you build your project
declare_id!("EKvZgSvPRUqT5wyVuMV2GAZnKG1MGTk2Tsz3y449p73H");

#[account]
pub struct Decimal {
    pub value: i128,
    pub decimals: u32,
}

impl Decimal {
    pub fn new(value: i128, decimals: u32) -> Self {
        Decimal { value, decimals }
    }
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut scaled_val = self.value.to_string();
        if scaled_val.len() <= self.decimals as usize {
            scaled_val.insert_str(
                0,
                &vec!["0"; self.decimals as usize - scaled_val.len()].join(""),
            );
            scaled_val.insert_str(0, "0.");
        } else {
            scaled_val.insert(scaled_val.len() - self.decimals as usize, '.');
        }
        f.write_str(&scaled_val)
    }
}

#[program]
pub mod chainlink_solana_demo {
    use super::*;
    pub fn execute(ctx: Context<Execute>) -> Result<()> {
        let feed = &ctx.accounts.chainlink_feed;

        // Read the feed data directly from the account (v2 SDK)
        let result = read_feed_v2(feed.try_borrow_data()?, feed.owner.to_bytes())
            .map_err(|_| DemoError::ReadError)?;

        // Get the latest round data
        let round = result
            .latest_round_data()
            .ok_or(DemoError::RoundDataMissing)?;

        let description = result.description();
        let decimals = result.decimals();

        // Convert description bytes to string
        let description_str = std::str::from_utf8(&description)
            .unwrap_or("Unknown")
            .trim_end_matches('\0');

        // write the latest price to the program output
        let decimal_print = Decimal::new(round.answer, u32::from(decimals));
        msg!("{} price is {}", description_str, decimal_print);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Execute<'info> {
    /// CHECK: We're reading data from this chainlink feed account
    pub chainlink_feed: AccountInfo<'info>,
}

#[error_code]
pub enum DemoError {
    #[msg("read error")]
    ReadError,
    #[msg("no round data")]
    RoundDataMissing,
}
