#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use pyth_sdk_solana::load_price_feed_from_account_info;
use std::{cell::RefCell, rc::Rc};

pub fn get_pyth_price_handler<'info>(
    mut pyth_price_account: UncheckedAccount<'info>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    let mut price_feed = {
        if pyth_price_account.key()
            != Pubkey::new_from_array([
                239u8, 13u8, 139u8, 111u8, 218u8, 44u8, 235u8, 164u8, 29u8, 161u8, 93u8, 64u8,
                149u8, 209u8, 218u8, 57u8, 42u8, 13u8, 47u8, 142u8, 208u8, 198u8, 199u8, 188u8,
                15u8, 76u8, 250u8, 200u8, 194u8, 128u8, 181u8, 109u8,
            ])
        {
            panic!("Pyth PriceAccount validation failed: expected mainnet-SOL/USD")
        }

        load_price_feed_from_account_info(&pyth_price_account).unwrap()
    };

    {
        let price = price_feed.get_price_unchecked();

        (price.price as f64) * 10f64.powf(price.expo as f64)
    };

    let mut price = price_feed.get_price_unchecked();
    let mut x = {
        let price = price;

        (price.price as f64) * 10f64.powf(price.expo as f64)
    };

    let mut p = price.price;
    let mut c = price.conf;
    let mut e = price.expo;

    solana_program::msg!("{} {}", "Pyth price: ".to_string(), x);

    solana_program::msg!("{} {}", "Pyth price: ".to_string(), p);

    solana_program::msg!("{} {}", "Pyth confidence interval: ".to_string(), c);

    solana_program::msg!("{} {}", "Pyth account decimal exponent: ".to_string(), e);
}
