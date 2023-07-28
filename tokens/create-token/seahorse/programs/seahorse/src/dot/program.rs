#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

pub fn init_token_account_handler<'info>(
    mut new_token_account: Empty<SeahorseAccount<'info, '_, TokenAccount>>,
    mut mint: SeahorseAccount<'info, '_, Mint>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    new_token_account.account.clone();
}

pub fn init_token_mint_handler<'info>(
    mut new_token_mint: Empty<SeahorseAccount<'info, '_, Mint>>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    new_token_mint.account.clone();
}

pub fn use_token_mint_handler<'info>(
    mut mint: SeahorseAccount<'info, '_, Mint>,
    mut recipient: SeahorseAccount<'info, '_, TokenAccount>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    solana_program::msg!(
        "{} {:?}",
        "Before mint Owner token : ".to_string(),
        recipient.amount
    );

    token::mint_to(
        CpiContext::new(
            mint.programs.get("token_program"),
            token::MintTo {
                mint: mint.to_account_info(),
                authority: signer.clone().to_account_info(),
                to: recipient.clone().to_account_info(),
            },
        ),
        <u64 as TryFrom<_>>::try_from(3000).unwrap(),
    )
    .unwrap();

    solana_program::msg!(
        "{} {:?}",
        "After mint Owner token : ".to_string(),
        recipient.amount
    );
}
