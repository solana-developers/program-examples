use std::{fmt::{self, Display}, str::FromStr};

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ABWallet {
    pub wallet: Pubkey,
    pub allowed: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Mode {
    Allow,
    Block,
    Mixed
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Mode, ()> {
        match s {
            "Allow" => Ok(Mode::Allow),
            "Block" => Ok(Mode::Block),
            "Mixed" => Ok(Mode::Mixed),
            _ => Err(()),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Allow => write!(f, "Allow"),
            Mode::Block => write!(f, "Block"),
            Mode::Mixed => write!(f, "Mixed"),
        }
    }
}

