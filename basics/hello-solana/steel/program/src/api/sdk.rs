use super::prelude::*;
use steel::*;

pub fn hello(signer: Pubkey) -> Instruction {
    Instruction {
        program_id: ID,
        accounts: vec![AccountMeta::new(signer, true)],
        data: HelloSolana {}.to_bytes(),
    }
}
