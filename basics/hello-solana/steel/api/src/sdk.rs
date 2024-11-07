use steel::*;

use crate::prelude::*;

pub fn hello(signer: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![AccountMeta::new(signer, true)],
        data: HelloSolana {}.to_bytes(),
    }
}

