use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum ProgramDerivedAddressesAccount {
    PageVisits = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct PageVisits {
    pub page_visits: [u8; 4], // u32
    pub bump: [u8; 1],
}

impl PageVisits {
    pub fn increment_visits(&mut self) {
        let visits = u32::from_le_bytes(self.page_visits);
        self.page_visits = (visits + 1).to_le_bytes();
    }

    pub fn page_visits(&self) -> u32 {
        u32::from_le_bytes(self.page_visits)
    }
}

account!(ProgramDerivedAddressesAccount, PageVisits);

/// Fetch PDA of the PageVisit account.
pub fn page_visits_pda(user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[SEED, user.as_ref()], &crate::id())
}
