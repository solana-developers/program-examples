pub mod instruction;
pub mod macros;

use instruction::*;
use steel::*;

pub mod prelude {
    pub use crate::instruction::*;
}

// TODO Set program id
declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");
