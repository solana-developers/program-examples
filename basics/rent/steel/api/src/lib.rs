pub mod error;
pub mod instruction;
pub mod state;
pub mod sdk;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::instruction::*;
    pub use crate::state::*; 
    pub use crate::sdk::*;
    // Re-export common solana dependencies
    pub use solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
        rent::Rent,
        system_program,
    };
}

use steel::*;

// TODO Set program id
declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35"); 
