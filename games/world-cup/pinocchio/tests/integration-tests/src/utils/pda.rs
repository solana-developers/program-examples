use solana_pubkey::Pubkey;

use crate::{
    state::common::{CONFIG_SEED, ORACLE_SEED, VAULT_SEED},
    tests::constants::PROGRAM_ID,
    Bracket,
};

pub fn get_config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG_SEED], &PROGRAM_ID)
}

pub fn get_oracle_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ORACLE_SEED], &PROGRAM_ID)
}

pub fn get_vault_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[VAULT_SEED], &PROGRAM_ID)
}

pub fn get_bracket_pda(owner: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[Bracket::SEED, owner.as_ref()], &PROGRAM_ID)
}
