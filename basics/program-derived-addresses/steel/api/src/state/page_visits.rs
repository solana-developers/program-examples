use super::SteelAccount;
use steel::*;

account!(SteelAccount, PageVisits);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct PageVisits {
    pub page_visits: u32,
    pub bump: u8,
}

impl PageVisits {
    pub const SEED_PREFIX: &'static str = "page_visits";

    pub fn increment(&mut self) {
        self.page_visits += 1;
    }
}
