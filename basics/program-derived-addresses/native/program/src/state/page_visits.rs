use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct IncrementPageVisits {}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PageVisits {
    pub page_visits: u32,
    pub bump: u8,
}

impl PageVisits {
    pub const ACCOUNT_SPACE: usize = 8 + 32;

    pub const SEED_PREFIX: &'static str = "page_visits";

    pub fn new(page_visits: u32, bump: u8) -> Self {
        PageVisits { page_visits, bump }
    }

    pub fn increment(&mut self) {
        self.page_visits += 1;
    }
}
