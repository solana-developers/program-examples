#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

pub fn create_associated_token_account_handler<'info>(
    mut token_account: Empty<SeahorseAccount<'info, '_, TokenAccount>>,
    mut mint: SeahorseAccount<'info, '_, Mint>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    token_account.account.clone();
}

pub fn create_token_handler<'info>(
    mut mint: Empty<SeahorseAccount<'info, '_, Mint>>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    mint.account.clone();
}

pub fn mint_token_handler<'info>(
    mut mint: SeahorseAccount<'info, '_, Mint>,
    mut recipient: SeahorseAccount<'info, '_, TokenAccount>,
    mut signer: SeahorseSigner<'info, '_>,
    mut amount: u64,
) -> () {
    token::mint_to(
        CpiContext::new(
            mint.programs.get("token_program"),
            token::MintTo {
                mint: mint.to_account_info(),
                authority: signer.clone().to_account_info(),
                to: recipient.clone().to_account_info(),
            },
        ),
        (amount
            * <u64 as TryFrom<_>>::try_from(10)
                .unwrap()
                .pow(<u32 as TryFrom<_>>::try_from(mint.decimals.clone()).unwrap())),
    )
    .unwrap();
}

pub fn transfer_handler<'info>(
    mut signer_token_account: SeahorseAccount<'info, '_, TokenAccount>,
    mut recipient: SeahorseAccount<'info, '_, TokenAccount>,
    mut signer: SeahorseSigner<'info, '_>,
    mut amount: u64,
    mut mint: SeahorseAccount<'info, '_, Mint>,
) -> () {
    if !(signer_token_account.mint == mint.key()) {
        panic!("Mint is not the token account mint");
    }

    token::transfer(
        CpiContext::new(
            signer_token_account.programs.get("token_program"),
            token::Transfer {
                from: signer_token_account.to_account_info(),
                authority: signer.clone().to_account_info(),
                to: recipient.clone().to_account_info(),
            },
        ),
        (amount
            * <u64 as TryFrom<_>>::try_from(10)
                .unwrap()
                .pow(<u32 as TryFrom<_>>::try_from(mint.decimals.clone()).unwrap())),
    )
    .unwrap();
}
