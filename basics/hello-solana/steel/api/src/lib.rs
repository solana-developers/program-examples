pub mod error;
pub mod instruction;
pub mod sdk;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::instruction::*;
    pub use crate::sdk::*;
}

use steel::*;

// TODO Set program id
declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");
