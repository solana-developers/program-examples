use steel::*;

use crate::prelude::*;

pub fn create(signer: Pubkey, user: Pubkey, page_visits: PageVisits) -> Instruction {
    let pda = page_visits_pda(&user);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(user, false),
            AccountMeta::new(pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Create { page_visits }.to_bytes(),
    }
}

pub fn increment(page_visits_pda: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![AccountMeta::new(page_visits_pda, false)],
        data: Increment {}.to_bytes(),
    }
}
