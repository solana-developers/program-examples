use steel::*;

use crate::prelude::*;

pub fn create_amm(payer: Pubkey, admin: Pubkey, id: Pubkey, fee: u16) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(admin, false),
            AccountMeta::new(amm_pda(id).0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CreateAmm {
            id,
            fee: fee.to_be_bytes(),
        }
        .to_bytes(),
    }
}

// pub fn add(signer: Pubkey, amount: u64) -> Instruction {
//     Instruction {
//         program_id: crate::ID,
//         accounts: vec![
//             AccountMeta::new(signer, true),
//             AccountMeta::new(counter_pda().0, false),
//         ],
//         data: Add {
//             amount: amount.to_le_bytes(),
//         }
//         .to_bytes(),
//     }
// }
