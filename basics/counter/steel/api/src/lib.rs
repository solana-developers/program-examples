pub mod consts;
pub mod instruction;
pub mod state;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::instruction::*;
    pub use crate::state::*;
}

use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");
