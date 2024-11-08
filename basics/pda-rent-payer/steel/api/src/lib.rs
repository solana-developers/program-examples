mod instruction;
mod state;

use instruction::*;
use steel::*;

pub mod prelude {
    pub use crate::instruction::*;
    pub use crate::state::*;
}

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");
