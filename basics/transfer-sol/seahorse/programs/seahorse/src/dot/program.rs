#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct MockAccount {
    pub mock_value: u8,
}

impl<'info, 'entrypoint> MockAccount {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedMockAccount<'info, 'entrypoint>> {
        let mock_value = account.mock_value;

        Mutable::new(LoadedMockAccount {
            __account__: account,
            __programs__: programs_map,
            mock_value,
        })
    }

    pub fn store(loaded: Mutable<LoadedMockAccount>) {
        let mut loaded = loaded.borrow_mut();
        let mock_value = loaded.mock_value;

        loaded.__account__.mock_value = mock_value;
    }
}

#[derive(Debug)]
pub struct LoadedMockAccount<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, MockAccount>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub mock_value: u8,
}

pub fn init_mock_account_handler<'info>(
    mut signer: SeahorseSigner<'info, '_>,
    mut mock_account: Empty<Mutable<LoadedMockAccount<'info, '_>>>,
) -> () {
    let mut account = mock_account.account.clone();

    assign!(
        account.borrow_mut().mock_value,
        <u8 as TryFrom<_>>::try_from(0).unwrap()
    );
}

pub fn transfer_sol_with_cpi_handler<'info>(
    mut sender: SeahorseSigner<'info, '_>,
    mut recipient: Mutable<LoadedMockAccount<'info, '_>>,
    mut amount: u64,
) -> () {
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &sender.key(),
            &recipient.borrow().__account__.key(),
            amount.clone(),
        ),
        &[
            sender.to_account_info(),
            recipient.borrow().__account__.to_account_info(),
            sender.programs.get("system_program").clone(),
        ],
    )
    .unwrap();
}
