#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct PageVisits {
    pub visits: u32,
}

impl<'info, 'entrypoint> PageVisits {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedPageVisits<'info, 'entrypoint>> {
        let visits = account.visits;

        Mutable::new(LoadedPageVisits {
            __account__: account,
            __programs__: programs_map,
            visits,
        })
    }

    pub fn store(loaded: Mutable<LoadedPageVisits>) {
        let mut loaded = loaded.borrow_mut();
        let visits = loaded.visits;

        loaded.__account__.visits = visits;
    }
}

#[derive(Debug)]
pub struct LoadedPageVisits<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, PageVisits>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub visits: u32,
}

pub fn create_page_visits_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut page_visits: Empty<Mutable<LoadedPageVisits<'info, '_>>>,
) -> () {
    page_visits.account.clone();
}

pub fn increment_page_visits_handler<'info>(
    mut page_visits: Mutable<LoadedPageVisits<'info, '_>>,
) -> () {
    assign!(
        page_visits.borrow_mut().visits,
        page_visits.borrow().visits + 1
    );
}
