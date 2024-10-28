#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Message {
    pub owner: Pubkey,
    pub value: String,
}

impl<'info, 'entrypoint> Message {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedMessage<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let value = account.value.clone();

        Mutable::new(LoadedMessage {
            __account__: account,
            __programs__: programs_map,
            owner,
            value,
        })
    }

    pub fn store(loaded: Mutable<LoadedMessage>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let value = loaded.value.clone();

        loaded.__account__.value = value;
    }
}

#[derive(Debug)]
pub struct LoadedMessage<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Message>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub value: String,
}

pub fn hello_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut message: Mutable<LoadedMessage<'info, '_>>,
) -> () {
    if !(owner.key() == message.borrow().owner) {
        panic!("This is not your message");
    }

    assign!(message.borrow_mut().value, "Hello GM!".to_string());
}

pub fn initialize_handler<'info>(
    mut authority: SeahorseSigner<'info, '_>,
    mut message: Empty<Mutable<LoadedMessage<'info, '_>>>,
) -> () {
    let mut message = message.account.clone();

    assign!(message.borrow_mut().owner, authority.key());

    assign!(message.borrow_mut().value, "".to_string());
}
