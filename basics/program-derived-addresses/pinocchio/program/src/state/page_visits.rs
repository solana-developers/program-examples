pub struct IncrementPageVisits {}

pub struct PageVisits {
    pub page_visits: u32,
    pub bump: u8,
}

impl PageVisits {
    pub const ACCOUNT_SPACE: usize = 4 + 1;

    pub const SEED_PREFIX: &'static str = "page_visits";

    pub fn new(page_visits: u32, bump: u8) -> Self {
        PageVisits { page_visits, bump }
    }
}
