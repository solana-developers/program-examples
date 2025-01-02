use steel::*;

use crate::prelude::*;

pub fn go_to_the_park(signer: Pubkey, data: GoToTheParkData) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![AccountMeta::new(signer, true)],
        data: GoToThePark { data }.to_bytes(),
    }
}
