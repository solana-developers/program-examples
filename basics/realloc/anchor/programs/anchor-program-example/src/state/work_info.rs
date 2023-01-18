use anchor_lang::prelude::*;


#[account]
pub struct WorkInfo {
    pub name: String,
    pub position: String,
    pub company: String,
    pub years_employed: u8,
}

impl WorkInfo {

    pub const ACCOUNT_SPACE: usize = 8 + 40 + 40 + 40 + 1;

    pub fn new(
        name: String,
        position: String,
        company: String,
        years_employed: u8,
    ) -> Self {

        WorkInfo {
            name,
            position,
            company,
            years_employed,
        }
    }
}