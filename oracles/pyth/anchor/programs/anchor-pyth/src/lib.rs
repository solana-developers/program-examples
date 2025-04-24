use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use anchor_lang::prelude::*;

declare_id!("46T9wfa7dRLJwMmMtLnzQji1B2ydjzm28uxQuhn9p9sR");

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
    pub price_update: Account<'info, PriceUpdateV2>,
}
