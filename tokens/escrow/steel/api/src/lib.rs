pub mod instruction;
pub mod state;

pub use instruction::*;
use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

pub mod prelude {
    pub use crate::instruction::*;
    pub use crate::state::*;
}
